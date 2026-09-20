use super::*;
use arc_swap::ArcSwap;
use std::sync::{Arc, OnceLock};

pub static DYNAMIC_LIBRARY: OnceLock<Arc<ArcSwap<LibraryWrapper>>> = OnceLock::new();

#[doc(hidden)]
pub fn provide_fn<F>(library_file_name: &'static str, symbol_name: &'static [u8]) -> FnGuard<F> {
    let library = provide_library_wrapper(library_file_name);
    let fn_guard = library.get(symbol_name);
    return fn_guard;
}

#[doc(hidden)]
pub fn provide_library_wrapper(library_file_name: &str) -> Arc<LibraryWrapper> {
    let library = DYNAMIC_LIBRARY
        .get_or_init(|| {
            let library_source_path = std::env::current_exe()
                .unwrap_or_else(|e| panic!("Unable to get current exe path: {e:?}"))
                .parent()
                .unwrap_or_else(|| panic!("Unable to get current exe parent path: parent is empty"))
                .join(library_file_name);

            let shared_library_wrapper = Arc::new(ArcSwap::from_pointee(LibraryWrapper::new(
                library_source_path.clone(),
            )));

            library_watcher::spawn(library_source_path, shared_library_wrapper.clone());
            return shared_library_wrapper;
        })
        .load_full();
    return library;
}
