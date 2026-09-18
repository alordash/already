const PROFILE_VAR: &str = "PROFILE";
const DEBUG_PROFILE: &str = "debug";

pub fn is_debug() -> bool {
    if let Ok(profile) = std::env::var(PROFILE_VAR)
        && profile == DEBUG_PROFILE
    {
        return true;
    }
    return false;
}
