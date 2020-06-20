mod algorithms;
mod util;

use std::env;
use std::fs;

fn main() {
    env_logger::init();
    let mut simple_alg = algorithms::simple::Simple::new();
    for i in 1..9 {
        let base_path = env::var("BASE_PATH").unwrap();
        let path = format!("{base}/{value}.jpeg", base=base_path, value=i);
        println!("{}", path);
        let bytes = fs::read(path).unwrap();
        match simple_alg.ingest(&bytes) {
            Ok(()) => {
                println!("GOOD");
            },
            Err(msg) => {
                println!("err:{}", msg.to_string());
            }
        }
    }
}
