use super::*;
use arc_swap::ArcSwap;
use notify::Watcher;
use std::sync::{Arc, OnceLock};

static DYNAMIC_LIBRARY: OnceLock<Arc<ArcSwap<RuntimeLibraryWrapper>>> = OnceLock::new();

pub fn provide_fn<F>(library_file_name: &'static str, symbol_name: &'static [u8]) -> FnGuard<F> {
    let library = DYNAMIC_LIBRARY
        .get_or_init(|| {
            let library_source_path = std::env::current_exe()
                .unwrap_or_else(|e| panic!("Unable to get current exe path: {e:?}"))
                .parent()
                .unwrap_or_else(|| panic!("Unable to get current exe parent path: parent is empty"))
                .to_owned()
                .join(library_file_name);

            let shared_library_wrapper = Arc::new(ArcSwap::from_pointee(
                RuntimeLibraryWrapper::new(library_source_path.clone()),
            ));

            library_watcher::spawn(library_source_path, shared_library_wrapper.clone());
            return shared_library_wrapper;
        })
        .load_full();
    let fn_guard = library.get(symbol_name);
    return fn_guard;
}
