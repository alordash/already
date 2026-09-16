pub struct SimpleLibraryWrapper {
    library_file_name: &'static str,
    inner: libloading::Library,
}

impl SimpleLibraryWrapper {
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
            inner,
        };
        return result;
    }

    pub fn get<T: Copy>(&self, symbol: &'static [u8]) -> T {
        let result = unsafe {
            self.inner.get(symbol).map(|x| *x).unwrap_or_else(|e| {
                panic!(
                    "Error finding symbol '{}', in shared library '{}': {:?}",
                    String::from_utf8_lossy(symbol),
                    self.library_file_name,
                    e
                )
            })
        };
        return result;
    }
}
