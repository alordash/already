use super::*;
use arc_swap::ArcSwap;
use notify::Watcher;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

pub struct LibrariesCache {
    inner: GrowHashMap<&'static str, Arc<ArcSwap<RuntimeLibraryWrapper>>>,
}

impl LibrariesCache {
    pub fn new() -> Self {
        Self {
            inner: GrowHashMap::new(),
        }
    }

    pub fn get(&self, library_file_name: &'static str) -> Arc<RuntimeLibraryWrapper> {
        let result = self.inner.get_or_insert_with(library_file_name, || {
            let shared_library_wrapper = Arc::new(ArcSwap::from_pointee(
                RuntimeLibraryWrapper::new(library_file_name),
            ));
            spawn_watcher(library_file_name, shared_library_wrapper.clone());
            return shared_library_wrapper;
        });
        return result.load_full();
    }
}

fn spawn_watcher(
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

    watcher
        .watch(
            &PathBuf::from_str(library_file_name).expect("TODO: Unable to get library path"),
            notify::RecursiveMode::NonRecursive,
        )
        .unwrap();

    core::mem::forget(watcher);
}
