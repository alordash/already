use crate::LibraryWrapper;
use crate::static_library::DYNAMIC_LIBRARY;
use arc_swap::ArcSwap;
use notify_debouncer_full::{DebounceEventResult, notify};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

pub fn spawn(library_path: PathBuf, shared_library_wrapper: Arc<ArcSwap<LibraryWrapper>>) {
    let static_library_path: &Path = library_path.leak();
    let mut watcher = notify_debouncer_full::new_debouncer(
        Duration::from_millis(100),
        None,
        move |events_result: DebounceEventResult| {
            let Some(current_lib) = DYNAMIC_LIBRARY.get().map(|x| x.load()) else {
                return;
            };
            let debounced_events = events_result.unwrap_or_else(|e| {
                panic!("Error handling debounced library file update event: {e:?}")
            });
            let current_lib_path = current_lib.library_copy_path();

            let mut lib_was_changed = false;
            for event in debounced_events.into_iter().map(|x| x.event) {
                let notify::Event {
                    kind: notify::EventKind::Create(_) | notify::EventKind::Modify(_),
                    paths,
                    ..
                } = event
                else {
                    continue;
                };
                for path in paths.into_iter() {
                    if path == current_lib_path {
                        return;
                    } else if path == static_library_path {
                        lib_was_changed = true;
                    }
                }
            }
            if !lib_was_changed {
                return;
            }
            let new_library = LibraryWrapper::new(static_library_path.to_owned());
            shared_library_wrapper.store(Arc::new(new_library));
        },
    )
    .unwrap();

    let parent_library_path = static_library_path.parent().unwrap_or_else(|| {
        panic!("Unable to get parent directory of library path '{static_library_path:?}'.")
    });
    watcher
        .watch(parent_library_path, notify::RecursiveMode::NonRecursive)
        .unwrap();

    core::mem::forget(watcher);
}
