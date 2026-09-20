use crate::LibraryWrapper;
use arc_swap::ArcSwap;
use notify::Watcher;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub fn spawn(library_path: PathBuf, shared_library_wrapper: Arc<ArcSwap<LibraryWrapper>>) {
    let static_library_path: &Path = library_path.leak();
    let mut watcher = notify::recommended_watcher(move |event| {
        match &event {
            Ok(notify::Event {
                kind: notify::EventKind::Create(_) | notify::EventKind::Modify(_),
                paths,
                ..
            }) if paths.iter().any(|x| x == static_library_path) => {}
            _ => return,
        }
        println!("RELOADING");
        let new_library = LibraryWrapper::new(static_library_path.to_owned());
        shared_library_wrapper.store(Arc::new(new_library));
    })
    .unwrap();

    let parent_library_path = static_library_path.parent().unwrap_or_else(|| {
        panic!("Unable to get parent directory of library path '{static_library_path:?}'.")
    });
    watcher
        .watch(parent_library_path, notify::RecursiveMode::NonRecursive)
        .unwrap();

    core::mem::forget(watcher);
}
