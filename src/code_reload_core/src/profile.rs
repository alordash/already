const PROFILE_VAR: &'static str = "PROFILE";
const DEBUG_PROFILE: &'static str = "debug";

pub fn is_debug() -> bool {
    if let Ok(profile) = std::env::var(PROFILE_VAR)
        && profile == DEBUG_PROFILE
    {
        return true;
    }
    return false;
}
