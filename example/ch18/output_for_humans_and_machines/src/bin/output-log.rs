#[macro_use]
extern crate log;

fn main() {
    // RUST_LOG=info cargo run --bin output-log
    env_logger::init();
    info!("starting up");
    warn!("oops, nothing implemented!");
}
