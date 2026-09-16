mod runtime;
mod simple_library_wrapper;

use crate::runtime::*;
use arc_swap::ArcSwap;
pub use simple_library_wrapper::*;
use std::sync::Arc;

fn usage() {
    let w = ArcSwap::new(Arc::new(1));
    let f = provide_fn::<fn(i32) -> i32>("lib.dll", b"enis");
    let q = f(3);
}
