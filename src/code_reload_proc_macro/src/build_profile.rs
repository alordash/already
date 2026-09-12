const PROFILE_VAR: &'static str = "PROFILE";
const DEBUG_PROFILE: &'static str = "debug";

pub(crate) fn is_debug() -> bool {
    if let Ok(profile) = std::env::var(PROFILE_VAR)
        && profile == DEBUG_PROFILE
    {
        return true;
    }
    return false;
}
