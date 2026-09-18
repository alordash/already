mod dynamic_library_check;
mod runtime;
mod simple_library_wrapper;

pub use dynamic_library_check::is_outside_dynamic_library;
pub use runtime::provide_fn;
pub use simple_library_wrapper::*;

fn usage(v: i32) -> i32 {
    if !is_outside_dynamic_library(usage as *const _) {
        return provide_fn::<fn(i32) -> i32>("lib.dll", b"enis")(v);
    }
    // Base implementation
    v + 1
}
