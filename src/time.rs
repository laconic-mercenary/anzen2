use std::time::{SystemTime, UNIX_EPOCH};
use rand::Rng;

pub fn current_ts_millis() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_millis()
}

pub fn random_u64() -> u64 {
    rand::thread_rng().gen()
}
