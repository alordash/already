use std::path::{Path, PathBuf};
use std::sync::LazyLock;

const DIR_ENV_VAR_NAME: &str = "CARGO_MANIFEST_DIR";
static DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let result = std::env::var(DIR_ENV_VAR_NAME).unwrap_or_else(|e| panic!("Unable to read environment variable '{DIR_ENV_VAR_NAME}' to get caller crate manifest directory: {e:?}")).into();
    return result;
});

pub fn dir() -> &'static Path {
    &DIR
}
