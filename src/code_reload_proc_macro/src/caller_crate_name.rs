use std::sync::LazyLock;

const CALLER_CRATE_NAME_ENV_VAR_NAME: &str = "CARGO_PKG_NAME";
static CALLER_CRATE_NAME: LazyLock<String> = LazyLock::new(|| {
    std::env::var(CALLER_CRATE_NAME_ENV_VAR_NAME).unwrap_or_else(|e| {
        panic!(
            "Unable to read environment variable '{CALLER_CRATE_NAME_ENV_VAR_NAME}' to get caller crate name"
        )
    })
});

pub(crate) fn get() -> String {
    CALLER_CRATE_NAME.clone()
}
