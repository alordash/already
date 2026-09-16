mod runtime;
mod simple_library_wrapper;

use crate::runtime::*;
use arc_swap::ArcSwap;
pub use simple_library_wrapper::*;
use std::sync::Arc;

fn usage(v: i32) -> i32 {
    provide_fn::<fn(i32) -> i32>("lib.dll", b"enis")(v)
}
