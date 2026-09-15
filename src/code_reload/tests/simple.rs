#![allow(non_snake_case)]

use std::io::{Read, Write};
use std::path::PathBuf;
use tempfile::tempdir;

const CARGO_MANIFEST_DIR_VAR_NAME: &str = "CARGO_MANIFEST_DIR";

#[test]
fn SimpleHotreload_Works() -> std::io::Result<()> {
    // Arrange
    let code_reload_dir = PathBuf::from(std::env::var(CARGO_MANIFEST_DIR_VAR_NAME).unwrap_or_else(|e| {
        panic!(
            "Unable to read project root path from env var '{CARGO_MANIFEST_DIR_VAR_NAME}': {e:?}"
        )
    }));
    dbg!(&code_reload_dir);
    let source_simple_dir = code_reload_dir.join("tests").join("simple");
    let target_dir = tempdir()?;

    dbg!(&target_dir);
    dircpy::copy_dir(source_simple_dir, &target_dir)?;

    let target_toml = target_dir.path().join("Cargo.toml");
    let target_toml_content = std::fs::read_to_string(&target_toml)?;
    let new_target_toml_content = target_toml_content.replace(
        r#"code_reload = { path = "../.." }"#,
        &format!(r#"code_reload = {{ path = {code_reload_dir:?} }}"#),
    );
    std::fs::write(target_toml, new_target_toml_content)?;
    cargo_rebuild_in(&target_dir, "simple")?;

    let target_lib = target_dir.as_ref().join("src").join("lib.rs");

    // Act
    let mut run_process = std::process::Command::new("cargo")
        .args(["run", "--bin", "main"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .current_dir(&target_dir)
        .spawn()?;
    let run_process_stdout = run_process
        .stdout
        .as_mut()
        .expect("Cargo run process must have stdout");
    run_process_stdout.read(&mut [0u8])?;
    std::fs::write(
        target_lib,
        r#"#[code_reload::hotreload]
#[allow(unused)]
pub fn get() -> i32 { 2 }"#,
    )?;
    cargo_rebuild_in(&target_dir, "simple")?;
    let mut run_process_stdin = run_process
        .stdin
        .as_ref()
        .expect("Cargo run process must have stdin");
    run_process_stdin.write(b"\n")?;
    run_process_stdin.flush()?;
    let run_process_output = run_process.wait_with_output()?;
    if !run_process_output.status.success() {
        panic!(
            "Error running target bin, exit code: {:?}",
            run_process_output.status.code()
        );
    }

    // Assert
    Ok(())
}

fn cargo_rebuild_in<P: AsRef<std::path::Path>>(
    p: P,
    project_name: &'static str,
) -> std::io::Result<()> {
    std::process::Command::new("cargo")
        .args(["clean", "-p", project_name])
        .current_dir(&p)
        .status()?;
    std::process::Command::new("cargo")
        .args(["build", "--lib"])
        .current_dir(p)
        .status()?;
    Ok(())
}
