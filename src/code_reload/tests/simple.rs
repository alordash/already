#![allow(non_snake_case)]

mod build_utils;
use build_utils::*;

const CARGO_MANIFEST_DIR_VAR_NAME: &str = "CARGO_MANIFEST_DIR";

#[test]
fn SimpleHotreload_Works() -> std::io::Result<()> {
    // Arrange
    let target_dir = copy_test_project("simple")?;
    let target_lib = target_dir.as_ref().join("src").join("lib.rs");
    cargo_clean_rebuild_in(&target_dir, "simple")?;

    // Act
    let mut test_run_process = cargo_run_in(&target_dir)?;
    test_run_process.wait_for_input_from_stdout()?;
    std::fs::write(
        target_lib,
        r#"#[code_reload::hotreload]
#[allow(unused)]
pub fn get() -> i32 { 2 }"#,
    )?;
    cargo_clean_rebuild_in(&target_dir, "simple")?;
    test_run_process.send_enter_to_stdin()?;

    // Assert
    test_run_process.wait_for_successful_exit()?;
    Ok(())
}
