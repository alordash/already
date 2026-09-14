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

#[allow(unreachable_code)]
pub fn library_file_name() -> String {
    let name = NAME.deref();
    #[cfg(target_os = "windows")]
    {
        return format!("{name}.dll");
    }
    #[cfg(target_os = "linux")]
    {
        return format!("lib{name}.so");
    }
    panic!("OS '{}' is not supported.", std::env::consts::OS);
}
