use super::*;
use std::ops::Deref;
use std::sync::Arc;

// TODO - write test that checks that FnGuard's drop leads to LibraryWrapper drop if Arc is exhausted
pub struct FnGuard<F> {
    f: F,
    _library_arc: Arc<LibraryWrapper>,
}

impl<F> FnGuard<F> {
    pub fn new(f: F, library_arc: Arc<LibraryWrapper>) -> Self {
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
