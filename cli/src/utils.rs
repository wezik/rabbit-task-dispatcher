use log::debug;

pub fn get_env_var(token: &str, default: &str, should_default: bool) -> String {
    match std::env::var(token) {
        Ok(value) if value.trim().len() > 0 => value,
        Ok(_) => {
            debug!(
                "Environment variable '{}' not found, defaulted to '{}'",
                token, default
            );
            default.to_string()
        }
        Err(_) if should_default => {
            debug!(
                "Environment variable '{}' not found, defaulted to '{}'",
                token, default
            );
            default.to_string()
        }
        _ => panic!(
            "Environment variable '{}' not found and has to be set",
            token
        ),
    }
}
