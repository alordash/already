// TODO - unsloppify
use std::sync::LazyLock;

#[cfg(windows)]
mod windows;

#[cfg(unix)]
mod unix;

#[cfg(windows)]
static IS_DYNAMIC_LIBRARY: LazyLock<bool> = LazyLock::new(windows::slow_is_outside_dynamic_library);

#[cfg(unix)]
static IS_DLL: LazyLock<bool> = LazyLock::new(unix::slow_is_outside_dynamic_library);

pub fn is_outside_dynamic_library() -> bool {
    *IS_DYNAMIC_LIBRARY
}
