use std::ops::Deref;
use std::sync::LazyLock;

const NAME_ENV_VAR_NAME: &str = "CARGO_PKG_NAME";
static NAME: LazyLock<String> = LazyLock::new(|| {
    let result = std::env::var(NAME_ENV_VAR_NAME).unwrap_or_else(|e| {
        panic!(
            "Unable to read environment variable '{NAME_ENV_VAR_NAME}' to get caller crate name: {e:?}"
        )
    });
    return result;
});

pub fn library_file_name() -> String {
    let name = NAME.deref();
    return already_core::get_platform_library_file_name(name);
}
