use hotcode::*;
use std::sync::{Arc, Weak};

fn main() -> Result<(), std::io::Error> {
    let first_foo_value = lib_foo::get();
    let first_bar_value = lib_bar::get_both();
    assert_eq!(first_foo_value, 1);
    assert_eq!(first_bar_value, (1, 1));
    dbg!(first_foo_value, first_bar_value);

    let bar_library = get_weak_library("lib_bar");

    println!();
    let _sync = std::io::stdin().read_line(&mut String::new())?;

    wait_for_library_reload(bar_library);

    let second_foo_value = lib_foo::get();
    let second_bar_value = lib_bar::get_both();
    assert_eq!(second_foo_value, 1);
    assert_eq!(second_bar_value, (2, 1));
    dbg!(second_foo_value, second_bar_value);

    let foo_library = get_weak_library("lib_foo");
    let bar_library = get_weak_library("lib_bar");

    println!();
    let _sync = std::io::stdin().read_line(&mut String::new())?;

    wait_for_library_reload(foo_library);
    wait_for_library_reload(bar_library);

    let third_foo_value = lib_foo::get();
    let third_bar_value = lib_bar::get_both();
    assert_eq!(third_foo_value, 2);
    assert_eq!(third_bar_value, (2, 2));
    dbg!(third_foo_value, third_bar_value);

    Ok(())
}

fn get_weak_library(library_name: &'static str) -> Weak<LibraryWrapper> {
    Arc::downgrade(&provide_library_wrapper(&get_platform_library_file_name(
        library_name,
    )))
}

fn wait_for_library_reload(weak_library: Weak<LibraryWrapper>) {
    // Wait for old library to be updated and unloaded
    while weak_library.strong_count() > 0 {
        std::hint::spin_loop();
    }
}
