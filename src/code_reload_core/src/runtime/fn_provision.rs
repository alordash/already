use super::*;
use std::sync::LazyLock;

static LIBRARIES_CACHE: LazyLock<LibrariesCache> = LazyLock::new(LibrariesCache::new);

pub fn provide_fn<F>(library_file_name: &'static str, symbol_name: &'static [u8]) -> FnReadLock<F> {
    let library = LIBRARIES_CACHE.get(library_file_name);
    let f = library.get(symbol_name);
    let result = FnReadLock::new(f, library);
    return result;
}
