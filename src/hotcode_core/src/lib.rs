mod dynamic_library_check;
mod fn_guard;
mod grow_map;
mod library_watcher;
mod library_wrapper;
mod platform_library_name;
mod static_library;

use fn_guard::*;
use grow_map::*;
use library_wrapper::*;

pub use dynamic_library_check::is_outside_dynamic_library;
pub use platform_library_name::get_platform_library_file_name;
pub use static_library::{provide_fn, provide_library_wrapper};
