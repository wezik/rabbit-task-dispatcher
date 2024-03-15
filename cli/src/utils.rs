use log::debug;

pub fn get_env_var(token: &str, default: &str, should_default: bool) -> String {
    match std::env::var(token) {
        Ok(value) => value,
        Err(_) if should_default => {
            debug!(
                "Environment variable '{}' not found, defaulted to '{}'",
                token, default
            );
            default.to_string()
        }
        _ => panic!(
            "Environment variable '{}' not found and no default provided",
            token
        ),
    }
}
