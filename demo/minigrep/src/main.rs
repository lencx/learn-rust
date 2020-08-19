use std::{ env, process };
use minigrep::Config;

fn main() {
    let conf = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = minigrep::run(conf) {
        eprintln!("Application error: {}",  e);
        process::exit(1)
    };
}
