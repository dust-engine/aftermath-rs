#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

use std::ffi::c_void;

pub const GFSDK_Aftermath_Version_API: u32 = 0x0000214; // Version 2.20

pub type PFN_GFSDK_Aftermath_GpuCrashDumpCb = unsafe extern "C" fn(
    p_gpu_crash_dump: *const c_void,
    gpu_crash_dump_size: u32,
    p_user_data: *mut c_void,
);
pub type PFN_GFSDK_Aftermath_ShaderDebugInfoCb = unsafe extern "C" fn(
    p_shader_debug_info: *const c_void,
    shader_debug_info_size: u32,
    p_user_data: *mut c_void,
);
pub type PFN_GFSDK_Aftermath_ResolveMarkerCb = unsafe extern "C" fn(
    p_marker: *const c_void,
    p_user_data: *mut c_void,
    resolved_marker_data: *mut *mut c_void,
    marker_size: *mut u32,
);
pub type PFN_GFSDK_Aftermath_AddGpuCrashDumpDescription =
    unsafe extern "C" fn(key: u32, value: *const u8);
pub type PFN_GFSDK_Aftermath_GpuCrashDumpDescriptionCb = unsafe extern "C" fn(
    add_value: PFN_GFSDK_Aftermath_AddGpuCrashDumpDescription,
    p_user_data: *mut c_void,
);

extern "C" {
    pub fn GFSDK_Aftermath_EnableGpuCrashDumps(
        version: u32,
        watched_apis: u32,
        flags: u32,

        gpuCrashDumpCb: PFN_GFSDK_Aftermath_GpuCrashDumpCb,
        shaderDebugInfoCb: PFN_GFSDK_Aftermath_ShaderDebugInfoCb,
        descriptionCb: PFN_GFSDK_Aftermath_GpuCrashDumpDescriptionCb,
        resolveMarkerCb: PFN_GFSDK_Aftermath_ResolveMarkerCb,
        p_user_data: *mut c_void,
    );
    pub fn GFSDK_Aftermath_DisableGpuCrashDumps();
    pub fn GFSDK_Aftermath_GetCrashDumpStatus(status: &mut u32);
}
