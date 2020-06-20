cargo vendor > .cargo/config
cargo build --release
cargo install --path . --verbose
BASE_PATH=test/img RUST_LOG=trace INIT_PIXEL_DIFF=0.97 ./target/release/motion