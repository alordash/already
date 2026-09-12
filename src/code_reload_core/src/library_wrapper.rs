pub struct LibraryWrapper {
    library_name: &'static str,
    inner: libloading::Library,
}

impl LibraryWrapper {
    pub fn new(library_name: &'static str) -> Self {
        let dynamic_library_path = std::env::current_exe()
            .expect("Unable to get current executable path from `std::env::current_exe()`.")
            .parent()
            .expect("Unable to get parent directory of current executable")
            .to_owned();
        let inner = unsafe {
            libloading::Library::new(dynamic_library_path)
                .unwrap_or_else(|e| panic!("Error opening shared library '{library_name}': {e:?}"))
        };
        let result = Self {
            library_name,
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
                    self.library_name,
                    e
                )
            })
        };
        return result;
    }
}
