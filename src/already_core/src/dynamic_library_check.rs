// TODO - unsloppify, support linux
#![allow(unused)]
#![allow(clippy::upper_case_acronyms)]
#![allow(non_snake_case)]
use std::ptr;
use std::sync::OnceLock;

type HMODULE = *mut std::os::raw::c_void;
type LPVOID = *mut std::os::raw::c_void;

unsafe extern "system" {
    fn GetModuleHandleExW(dwFlags: u32, lpModuleName: *const u16, phModule: *mut HMODULE) -> i32;

    fn GetModuleFileNameW(hModule: HMODULE, lpFilename: *mut u16, nSize: u32) -> u32;
}

const GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS: u32 = 0x00000004;
const GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT: u32 = 0x00000002;

pub fn is_outside_dynamic_library() -> bool {
    static IS_DLL: OnceLock<bool> = OnceLock::new();
    *IS_DLL.get_or_init(|| !slow_is_inside_dynamic_library())
}

fn slow_is_inside_dynamic_library() -> bool {
    let mut h_module: HMODULE = ptr::null_mut();
    let flags =
        GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT;

    unsafe {
        if GetModuleHandleExW(
            flags,
            slow_is_inside_dynamic_library as *const u16,
            &mut h_module,
        ) != 0
            && !h_module.is_null()
        {
            let mut buffer = [0u16; 32768];
            let len = GetModuleFileNameW(h_module, buffer.as_mut_ptr(), buffer.len() as u32);
            if len > 0 {
                let dll_path = String::from_utf16_lossy(&buffer[..len as usize]);
                let current_exe = std::env::current_exe()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_default();
                // Compare path of containing module with main executable path
                return !dll_path.eq_ignore_ascii_case(&current_exe);
            }
        }
    }
    false
}
