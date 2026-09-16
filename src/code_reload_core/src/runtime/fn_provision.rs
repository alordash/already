use super::*;
use arc_swap::ArcSwap;
use notify::Watcher;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, LazyLock, OnceLock};

static DYNAMIC_LIBRARY: OnceLock<Arc<ArcSwap<RuntimeLibraryWrapper>>> = OnceLock::new();

pub fn provide_fn<F>(library_file_name: &'static str, symbol_name: &'static [u8]) -> FnReadLock<F> {
    let library = DYNAMIC_LIBRARY
        .get_or_init(|| {
            let shared_library_wrapper = Arc::new(ArcSwap::from_pointee(
                RuntimeLibraryWrapper::new(library_file_name),
            ));
            spawn_library_watcher(library_file_name, shared_library_wrapper.clone());
            return shared_library_wrapper;
        })
        .load_full();
    let f = library.get(symbol_name);
    let result = FnReadLock::new(f, library);
    return result;
}

fn spawn_library_watcher(
    library_file_name: &'static str,
    shared_library_wrapper: Arc<ArcSwap<RuntimeLibraryWrapper>>,
) {
    let mut watcher = notify::recommended_watcher(move |event| {
        match &event {
            Ok(notify::Event {
                kind: notify::EventKind::Create(_) | notify::EventKind::Modify(_),
                ..
            }) => {}
            _ => return,
        }
        let new_library = RuntimeLibraryWrapper::new(library_file_name);
        shared_library_wrapper.store(Arc::new(new_library));
    })
    .unwrap();

    let library_path = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join(library_file_name);
    dbg!(&library_path);
    watcher
        .watch(&library_path, notify::RecursiveMode::NonRecursive)
        .unwrap();

    core::mem::forget(watcher);
}
