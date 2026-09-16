use super::*;

pub struct RuntimeLibraryWrapper {
    library_file_name: &'static str,
    fn_ptrs_map: GrowHashMap<&'static [u8], *mut core::ffi::c_void>,
    inner: libloading::Library,
}

impl RuntimeLibraryWrapper {
    pub fn new(library_file_name: &'static str) -> Self {
        let dynamic_library_dir = std::env::current_exe()
            .expect("Unable to get current executable path from `std::env::current_exe()`.")
            .parent()
            .expect("Unable to get parent directory of current executable")
            .to_owned();
        let dynamic_library_path = dynamic_library_dir.join(library_file_name);
        let inner = unsafe {
            libloading::Library::new(dynamic_library_path).unwrap_or_else(|e| {
                panic!("Error opening shared library '{library_file_name}' in directory '{dynamic_library_dir:?}': {e:?}")
            })
        };
        let result = Self {
            library_file_name,
            fn_ptrs_map: GrowHashMap::new(),
            inner,
        };
        return result;
    }

    pub fn get<F>(&self, symbol_name: &'static [u8]) -> F {
        let raw_f = self.fn_ptrs_map.get_or_insert_with(symbol_name, || unsafe {
            self.inner
                .get::<fn()>(symbol_name)
                .unwrap()
                .try_as_raw_ptr()
                .unwrap()
        });
        let f_ptr = *raw_f as *const _ as *const ();
        let result = unsafe { core::mem::transmute_copy(&f_ptr) };
        return result;
    }
}

impl Drop for RuntimeLibraryWrapper {
    fn drop(&mut self) {
        // TODO - remove, it's only for debug
        dbg!("DROPPING RuntimeLibraryWrapper", self.library_file_name);
    }
}
