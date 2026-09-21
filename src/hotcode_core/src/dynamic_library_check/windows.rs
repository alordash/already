use winapi::shared::minwindef::*;
use winapi::um::libloaderapi::*;

pub fn slow_is_outside_dynamic_library() -> bool {
    let flags =
        GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS | GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT;
    let current_fn_ptr = slow_is_outside_dynamic_library as *const _;
    let mut h_module: HMODULE = core::ptr::null_mut();
    unsafe {
        if GetModuleHandleExW(flags, current_fn_ptr, &mut h_module) == 0 {
            return true;
        }
        if h_module.is_null() {
            return true;
        }
        let mut file_name_buffer = [0u16; 4096];
        let file_name_length = GetModuleFileNameW(
            h_module,
            file_name_buffer.as_mut_ptr(),
            file_name_buffer.len() as DWORD,
        );
        if file_name_length == 0 {
            return true;
        }
        let Ok(current_exe_path) = std::env::current_exe() else {
            return true;
        };

        let module_name_bytes = &file_name_buffer[..file_name_length as usize];
        let module_name = String::from_utf16_lossy(module_name_bytes);

        let result = current_exe_path.ends_with(module_name);
        return result;
    }
}
