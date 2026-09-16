mod grow_hash_map;
mod simple_library_wrapper;

use crate::grow_hash_map::GrowHashMap;
pub use simple_library_wrapper::*;
use std::ops::Deref;
use std::sync::{LazyLock, RwLock, RwLockReadGuard};

fn usage() {
    let f = get::<fn(i32) -> i32>("lib.dll", b"enis");
    let q = f(3);
}

struct LibrariesCache {
    inner: GrowHashMap<&'static str, RwLock<RuntimeLibraryWrapper>>,
}

impl LibrariesCache {
    pub fn get(&self, library_file_name: &'static str) -> &RwLock<RuntimeLibraryWrapper> {
        let result = self.inner.get_or_insert_with(library_file_name, || {
            RwLock::new(RuntimeLibraryWrapper {
                fn_ptrs_map: GrowHashMap::new(),
                inner: todo!("Load library"),
            })
        });
        return result;
    }
}

struct RuntimeLibraryWrapper {
    fn_ptrs_map: GrowHashMap<&'static [u8], *mut core::ffi::c_void>,
    inner: libloading::Library,
}

impl RuntimeLibraryWrapper {
    pub fn get<F>(&self, symbol_name: &'static [u8]) -> F {
        let raw_f = self.fn_ptrs_map.get_or_insert_with(symbol_name, || unsafe {
            self.inner
                .get::<()>(symbol_name)
                .unwrap()
                .try_as_raw_ptr()
                .unwrap()
        });
        let f_ptr = &raw_f as *const _ as *const ();
        let result = unsafe { core::mem::transmute_copy(&f_ptr) };
        return result;
    }
}

static LIBRARIES_MAP: LazyLock<LibrariesCache> = LazyLock::new(|| LibrariesCache {
    inner: GrowHashMap::new(),
});

fn get<'rwlock, F>(
    library_file_name: &'static str,
    symbol_name: &'static [u8],
) -> FnReadLock<'rwlock, F> {
    let library_lock = LIBRARIES_MAP.get(library_file_name).read().unwrap();
    let f = library_lock.get(symbol_name);
    let result = FnReadLock::new(f, library_lock);
    return result;
}

struct FnReadLock<'rwlock, F> {
    f: F,
    _library_lock: RwLockReadGuard<'rwlock, RuntimeLibraryWrapper>,
}

impl<'rwlock, F> FnReadLock<'rwlock, F> {
    pub fn new(f: F, library_lock: RwLockReadGuard<'rwlock, RuntimeLibraryWrapper>) -> Self {
        Self {
            f,
            _library_lock: library_lock,
        }
    }
}

impl<'rwlock, F> Deref for FnReadLock<'rwlock, F> {
    type Target = F;

    fn deref(&self) -> &Self::Target {
        &self.f
    }
}
