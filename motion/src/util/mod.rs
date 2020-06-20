use std::env;

pub fn get_env_as_f32(key:&str) -> f32 {
    let val = env::var(key).unwrap();
    val.parse::<f32>().unwrap()
}