// TODO - more test cases: where lib and bin are in same crate, where lib and bin are separate crate

mod build_utils;
use build_utils::*;

#[test]
fn hotreload_works() -> std::io::Result<()> {
    // Arrange
    const PACKAGE_NAME: &str = "lib_bin_together";
    let target_dir = copy_test_project(PACKAGE_NAME)?;
    let target_lib = target_dir.as_ref().join("src").join("lib.rs");
    let updated_lib = target_dir.as_ref().join("src").join("lib_updated.rs");
    cargo_clean_rebuild_in(target_dir.as_ref(), PACKAGE_NAME)?;

    // Act
    let old_lib_txt = std::fs::read_to_string(&target_lib)?;
    dbg!(old_lib_txt);
    
    let mut test_run_process = cargo_run_in(target_dir.as_ref(), "bin_together")?;
    test_run_process.wait_for_input_from_stdout()?;
    std::fs::copy(updated_lib, &target_lib)?;
    cargo_clean_rebuild_in(target_dir.as_ref(), PACKAGE_NAME)?;
    test_run_process.send_enter_to_stdin()?;
    
    let new_lib_txt = std::fs::read_to_string(&target_lib)?;
    dbg!(new_lib_txt);

    // Assert
    test_run_process.wait_for_successful_exit()?;
    Ok(())
}
