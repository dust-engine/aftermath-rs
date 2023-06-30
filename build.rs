#![feature(fs_try_exists)]

use std::{ffi::OsString, io::Write};

fn get_lib_name() -> &'static str {
    #[cfg(target_arch = "x86_64")]
    return "GFSDK_Aftermath_Lib.x64";

    #[cfg(target_arch = "x86")]
    return "GFSDK_Aftermath_Lib.x86";
    #[allow(unreachable_code)]
    {
        panic!()
    }
}

fn get_install_path() -> impl Iterator<Item = (&'static str, OsString)> {
    get_download_name().iter().map(|download_name| {
        let mut path: std::path::PathBuf = std::env::var("OUT_DIR").unwrap().into();
        path.push(download_name);
        (*download_name, path.into_os_string())
    })
}

fn get_download_name() -> &'static [&'static str] {
    #[cfg(target_family = "unix")]
    {
        #[cfg(target_arch = "x86_64")]
        return &["libGFSDK_Aftermath_Lib.x64.so"];

        #[cfg(target_arch = "x86")]
        return &["libGFSDK_Aftermath_Lib.x86.so"];
    }
    #[cfg(target_family = "windows")]
    {
        #[cfg(target_arch = "x86_64")]
        return &["GFSDK_Aftermath_Lib.x64.lib", "GFSDK_Aftermath_Lib.x64.dll"];

        #[cfg(target_arch = "x86")]
        return &["GFSDK_Aftermath_Lib.x86.lib", "GFSDK_Aftermath_Lib.x86.dll"];
    }
    #[allow(unreachable_code)]
    {
        panic!()
    }
}

fn main() {
    for (download_name, install_path) in get_install_path() {
        if !std::fs::try_exists(&install_path).expect("Unable to check library file location") {
            let data = sysreq::get(format!(
                "https://github.com/dust-engine/aftermath-rs/releases/download/v0.1/{}",
                download_name
            ))
            .expect("Download file error");
            let mut file =
                std::fs::File::create(install_path).expect("Unable to create library file");
            file.write_all(&data).expect("Unable to write library file");
        }
    }

    println!("cargo:rustc-link-lib={}", get_lib_name());
    println!(
        "cargo:rustc-link-search={}",
        std::env::var("OUT_DIR").unwrap()
    );
}
