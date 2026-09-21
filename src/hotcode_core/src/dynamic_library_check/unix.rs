use std::ffi::c_void;

// Define the libc dladdr binding structure
#[repr(C)]
pub struct DlInfo {
    pub dli_fname: *const libc::c_char,
    pub dli_fbase: *mut c_void,
    pub dli_sname: *const libc::c_char,
    pub dli_saddr: *mut c_void,
}

unsafe extern "C" {
    fn dladdr(addr: *const c_void, info: *mut DlInfo) -> std::os::raw::c_int;
}

pub fn slow_is_outside_dynamic_library() -> bool {
    let mut info: DlInfo = unsafe { std::mem::zeroed() };
    // Pass a pointer to our current function
    let current_fn_ptr = slow_is_outside_dynamic_library as *const c_void;

    unsafe {
        if dladdr(current_fn_ptr, &mut info) != 0 {
            // If dli_fname points to a file name and it's different from the main executable,
            // or we check the link map base address via more advanced methods, we can confirm.
            // A simple check is whether dli_fname is set and doesn't match the main program path.
            if !info.dli_fname.is_null() {
                let c_str = std::ffi::CStr::from_ptr(info.dli_fname);
                let path_str = c_str.to_string_lossy();
                // If it doesn't point to the main executable file, it's a dynamic library
                let current_exe = std::env::current_exe()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_default();
                return current_exe.ends_with(path_str.as_ref());
            }
        }
    }
    return true;
}
