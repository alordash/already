use super::*;
use std::ops::Deref;
use std::sync::Arc;

pub struct FnGuard<F> {
    f: F,
    _library_arc: Arc<RuntimeLibraryWrapper>,
}

impl<F> FnGuard<F> {
    pub fn new(f: F, library_arc: Arc<RuntimeLibraryWrapper>) -> Self {
        Self {
            f,
            _library_arc: library_arc,
        }
    }
}

impl<F> Deref for FnGuard<F> {
    type Target = F;

    fn deref(&self) -> &Self::Target {
        &self.f
    }
}

impl<F> Drop for FnGuard<F> {
    fn drop(&mut self) {
        // TODO - remove, it's only for debug
        println!(
            "Freed fn, library references count: {}, library ptr: {:?}",
            Arc::strong_count(&self._library_arc),
            Arc::as_ptr(&self._library_arc)
        );
    }
}
