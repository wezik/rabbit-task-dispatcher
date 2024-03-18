use log::debug;

pub fn read_env(token: &str, fallback: &str, should_fallback: bool) -> String {
    match std::env::var(token) {
        Ok(value) if value.trim().len() > 0 => value,
        Ok(_) => {
            debug!(
                "Environment variable '{}' not found, defaulted to '{}'",
                token, fallback
            );
            fallback.to_string()
        }
        Err(_) if should_fallback => {
            debug!(
                "Environment variable '{}' not found, defaulted to '{}'",
                token, fallback
            );
            fallback.to_string()
        }
        _ => panic!(
            "Environment variable '{}' not found and has to be set",
            token
        ),
    }
}
