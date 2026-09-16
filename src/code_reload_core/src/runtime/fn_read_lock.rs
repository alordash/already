use super::*;
use std::ops::Deref;
use std::sync::Arc;
use arc_swap::Guard;

pub struct FnReadLock<F> {
    f: F,
    _library_arc: Arc<RuntimeLibraryWrapper>,
}

impl<F> FnReadLock<F> {
    pub fn new(f: F, library_arc: Arc<RuntimeLibraryWrapper>) -> Self {
        Self {
            f,
            _library_arc: library_arc,
        }
    }
}

impl<F> Deref for FnReadLock<F> {
    type Target = F;

    fn deref(&self) -> &Self::Target {
        &self.f
    }
}
