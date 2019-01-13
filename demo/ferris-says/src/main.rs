extern crate ferris_says;

use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let out = b"Hello Rust --lencx";
    let width = 18;
    let mut wrirte = BufWriter::new(stdout.lock());
    say(out, width, &mut wrirte).unwrap();
}
