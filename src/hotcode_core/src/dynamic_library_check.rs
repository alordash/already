// TODO - unsloppify, support linux
#![allow(unused)]
#![allow(clippy::upper_case_acronyms)]
#![allow(non_snake_case)]
use std::ptr;
use std::sync::{LazyLock, OnceLock};

#[cfg(windows)]
mod windows;

#[cfg(unix)]
mod unix;

#[cfg(windows)]
static IS_DLL: LazyLock<bool> = LazyLock::new(windows::slow_is_outside_dynamic_library);

#[cfg(unix)]
static IS_DLL: LazyLock<bool> = LazyLock::new(unix::slow_is_outside_dynamic_library);

pub fn is_outside_dynamic_library() -> bool {
    *IS_DLL
}
