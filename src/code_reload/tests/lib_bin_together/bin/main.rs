use code_reload::*;
use lib_bin_together::*;
use std::sync::Arc;

fn main() -> Result<(), std::io::Error> {
    let old_values = get_values();
    let expected_old_values = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    assert_eq!(old_values, expected_old_values);

    let old_library = Arc::downgrade(&provide_library_wrapper(&get_platform_library_file_name(
        "lib_bin_together",
    )));

    println!();
    let _sync = std::io::stdin().read_line(&mut String::new())?;

    // Wait for old library to be updated and unloaded
    while old_library.strong_count() > 0 {
        std::hint::spin_loop();
    }

    let new_values = get_values();
    let expected_new_values = [10, 20, 30, 40, 50, 60, 70, 80, 90];
    assert_eq!(new_values, expected_new_values);

    Ok(())
}

fn get_values() -> [i32; 9] {
    [
        standalone_get(),
        Struct.get(),
        Struct.mut_get(),
        Struct.consume_get(),
        Struct::static_get(),
        Struct.trait_get(),
        Struct.trait_mut_get(),
        Struct.trait_consume_get(),
        Struct::trait_static_get(),
    ]
}
