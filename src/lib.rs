#![feature(ptr_metadata)]

use std::{
    ffi::c_void,
    ptr::{from_raw_parts_mut, Pointee},
};

mod sys;

pub struct Aftermath {
    user_data_ptr: *mut c_void,
}
unsafe impl Send for Aftermath {}
unsafe impl Sync for Aftermath {}

struct AftermathUserData {
    ptr: *mut (),
    meta: <dyn AftermathDelegate as Pointee>::Metadata,
}

impl Aftermath {
    pub fn new(delegate: impl AftermathDelegate) -> Self {
        let mut delegate = Box::new(delegate) as Box<dyn AftermathDelegate>;
        let ptr = delegate.as_mut() as *mut dyn AftermathDelegate;
        let (ptr, meta) = ptr.to_raw_parts();
        let mut user_data = Box::new(AftermathUserData { ptr, meta });
        let user_data_ptr = user_data.as_mut() as *mut _ as *mut c_void;
        Box::leak(user_data);
        Box::leak(delegate);

        unsafe {
            sys::GFSDK_Aftermath_EnableGpuCrashDumps(
                sys::GFSDK_Aftermath_Version_API,
                2, // Vulkan
                0,
                callbacks::GpuCrashDumpCallback,
                callbacks::ShaderDebugInfoCallback,
                callbacks::GpuCrashDumpDescriptionCallback,
                callbacks::ResolveMarkerCallback,
                user_data_ptr,
            );
        }
        Self { user_data_ptr }
    }
}
impl Drop for Aftermath {
    fn drop(&mut self) {
        unsafe {
            sys::GFSDK_Aftermath_DisableGpuCrashDumps();
            let user_data: Box<AftermathUserData> = Box::from_raw(self.user_data_ptr as *mut _);
            let ptr: *mut dyn AftermathDelegate = from_raw_parts_mut(user_data.ptr, user_data.meta);
            let delegate = Box::from_raw(ptr);
            drop(user_data);
            drop(delegate);
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    NotStarted = 0,
    CollectingData,
    CollectingDataFailed,
    InvokingCallback,
    Finished,
    Unknown,
}
impl Status {
    pub fn get() -> Self {
        let mut status: u32 = 0;
        unsafe {
            sys::GFSDK_Aftermath_GetCrashDumpStatus(&mut status);
            let status = status as u8;
            std::mem::transmute(status)
        }
    }
    pub fn wait_for_status(timeout: Option<std::time::Duration>) -> Self {
        let mut status = Self::get();
        let delta = std::time::Duration::from_millis(50);
        let mut time = std::time::Duration::new(0, 0);
        while status != Status::CollectingDataFailed
            && status != Status::Finished
            && timeout.map_or(true, |t| time < t)
        {
            std::thread::sleep(delta);
            time += delta;
            status = Self::get();
        }
        status
    }
}

pub trait AftermathDelegate: Send + Sync + 'static {
    fn dumped(&mut self, dump_data: &[u8]);
    fn shader_debug_info(&mut self, data: &[u8]);
    fn description(&mut self, describe: &mut DescriptionBuilder);
}

mod callbacks {
    #![allow(non_snake_case)]
    use std::{ffi::c_void, ptr::from_raw_parts_mut};

    use crate::{sys, AftermathDelegate, AftermathUserData};

    pub unsafe extern "C" fn GpuCrashDumpCallback(
        pGpuCrashDump: *const ::std::os::raw::c_void,
        gpuCrashDumpSize: u32,
        pUserData: *mut ::std::os::raw::c_void,
    ) {
        let user_data: &mut AftermathUserData = &mut *(pUserData as *mut AftermathUserData);
        let delegate: *mut dyn AftermathDelegate =
            from_raw_parts_mut(user_data.ptr, user_data.meta);
        let delegate = &mut *delegate;
        let crash_dump_bytes: &[u8] =
            std::slice::from_raw_parts(pGpuCrashDump as *const u8, gpuCrashDumpSize as usize);
        delegate.dumped(crash_dump_bytes);
    }
    pub unsafe extern "C" fn ShaderDebugInfoCallback(
        pData: *const ::std::os::raw::c_void,
        size: u32,
        pUserData: *mut ::std::os::raw::c_void,
    ) {
        let user_data: &mut AftermathUserData = &mut *(pUserData as *mut AftermathUserData);
        let delegate: *mut dyn AftermathDelegate =
            from_raw_parts_mut(user_data.ptr, user_data.meta);
        let delegate = &mut *delegate;
        let crash_dump_bytes: &[u8] = std::slice::from_raw_parts(pData as *const u8, size as usize);
        delegate.shader_debug_info(crash_dump_bytes);
    }
    pub unsafe extern "C" fn GpuCrashDumpDescriptionCallback(
        callback: sys::PFN_GFSDK_Aftermath_AddGpuCrashDumpDescription,
        pUserData: *mut c_void,
    ) {
        let user_data: &mut AftermathUserData = &mut *(pUserData as *mut AftermathUserData);
        let delegate: *mut dyn AftermathDelegate =
            from_raw_parts_mut(user_data.ptr, user_data.meta);
        let delegate = &mut *delegate;
        delegate.description(&mut crate::DescriptionBuilder(callback));
    }
    pub unsafe extern "C" fn ResolveMarkerCallback(
        _pMarker: *const c_void,
        _pUserData: *mut c_void,
        _resolvedMarkerData: *mut *mut c_void,
        _marker_size: *mut u32,
    ) {
    }
}

pub struct DescriptionBuilder(sys::PFN_GFSDK_Aftermath_AddGpuCrashDumpDescription);
impl DescriptionBuilder {
    pub fn set_application_name(&mut self, name: &str) {
        unsafe {
            (self.0)(1, name.as_ptr());
        }
    }
    pub fn set_application_version(&mut self, name: &str) {
        unsafe {
            (self.0)(2, name.as_ptr());
        }
    }
    pub fn set(&mut self, index: u32, name: &str) {
        unsafe {
            (self.0)(0x10000 + index, name.as_ptr());
        }
    }
}
