#[doc(hidden)]
#[allow(unreachable_code)]
pub fn get_platform_library_file_name(library_name: &str) -> String {
    #[cfg(windows)]
    {
        return format!("{library_name}.dll");
    }
    #[cfg(target_os = "macos")]
    {
        return format!("lib{library_name}.dylib");
    }
    #[cfg(unix)]
    {
        return format!("lib{library_name}.so");
    }
    panic!("OS '{}' is not supported.", std::env::consts::OS);
}
