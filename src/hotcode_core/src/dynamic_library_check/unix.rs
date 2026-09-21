use std::ffi::c_void;
use std::os::unix::ffi::OsStrExt;

pub fn slow_is_outside_dynamic_library() -> bool {
    let mut info = libc::Dl_info {
        dli_fname: core::ptr::null(),
        dli_fbase: core::ptr::null_mut(),
        dli_sname: core::ptr::null(),
        dli_saddr: core::ptr::null_mut(),
    };

    let current_fn_ptr = slow_is_outside_dynamic_library as *const c_void;

    unsafe {
        if libc::dladdr(current_fn_ptr, &mut info) == 0 {
            return true;
        }
        if info.dli_fname.is_null() {
            return true;
        }
        let Ok(current_exe_path) = std::env::current_exe() else {
            return true;
        };

        let c_str = std::ffi::CStr::from_ptr(info.dli_fname);
        let result = current_exe_path
            .as_os_str()
            .as_bytes()
            .ends_with(c_str.to_bytes());
        return result;
    }
}
