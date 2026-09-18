use crate::runtime::runtime_library_wrapper::RuntimeLibraryWrapper;
use arc_swap::ArcSwap;
use notify::Watcher;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub fn spawn(library_path: PathBuf, shared_library_wrapper: Arc<ArcSwap<RuntimeLibraryWrapper>>) {
    let static_library_path: &Path = library_path.leak();
    let mut watcher = notify::recommended_watcher(move |event| {
        match &event {
            Ok(notify::Event {
                kind: notify::EventKind::Create(_) | notify::EventKind::Modify(_),
                ..
            }) => {}
            _ => return,
        }
        println!("Updating library '{static_library_path:?}'");
        let new_library = RuntimeLibraryWrapper::new(static_library_path.to_owned());
        shared_library_wrapper.store(Arc::new(new_library));
        println!("Updated library '{static_library_path:?}'");
    })
    .unwrap();

    dbg!(&static_library_path);
    watcher
        .watch(&static_library_path, notify::RecursiveMode::NonRecursive)
        .unwrap();

    core::mem::forget(watcher);
}
