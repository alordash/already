use hotcode::*;
use lib_bin_together::*;
use std::sync::Arc;

fn main() -> Result<(), std::io::Error> {
    let old_values = get_values();
    let expected_old_values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    dbg!(old_values);
    assert_eq!(old_values, expected_old_values);

    let old_library = Arc::downgrade(&provide_library_wrapper(&get_platform_library_file_name(
        "lib_bin_together",
    )));

    println!();
    dbg!(old_library.strong_count(), old_library.as_ptr());
    let _sync = std::io::stdin().read_line(&mut String::new())?;

    // Wait for old library to be updated and unloaded
    dbg!(old_library.strong_count(), old_library.as_ptr());
    while old_library.strong_count() > 0 {
        std::hint::spin_loop();
    }
    dbg!(old_library.strong_count(), old_library.as_ptr());
    std::thread::sleep(std::time::Duration::from_millis(1000));

    let new_library = Arc::downgrade(&provide_library_wrapper(&get_platform_library_file_name(
        "lib_bin_together",
    )));
    dbg!(new_library.strong_count(), new_library.as_ptr());

    let new_values = get_values();
    let expected_new_values = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110];
    dbg!(new_values);
    assert_eq!(new_values, expected_new_values);

    Ok(())
}

fn get_values() -> [i32; 11] {
    [
        standalone_get(),
        Struct.get(),
        Struct.mut_get(),
        Struct.consume_get(),
        Box::new(Struct).box_get(),
        Struct::static_get(),
        Struct.trait_get(),
        Struct.trait_mut_get(),
        Struct.trait_consume_get(),
        Box::new(Struct).trait_box_get(),
        Struct::trait_static_get(),
    ]
}
