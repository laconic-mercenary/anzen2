use std::env;

const ENV_BIND_LOCALHOST_ADDR: &str = "BIND_LOCALHOST_ADDR";

pub fn bind_localhost_addr() -> bool {
    if let Ok(value) = env::var(ENV_BIND_LOCALHOST_ADDR) {
        match value.parse::<bool>() {
            Ok(parsed) => parsed,
            Err(_) => {
                log::warn!("{} env var is not a valid boolean, defaulting to true", ENV_BIND_LOCALHOST_ADDR);
                true
            }
        }
    } else {
        log::warn!("{} env var is not set, defaulting to true", ENV_BIND_LOCALHOST_ADDR);
        true
    }
}
