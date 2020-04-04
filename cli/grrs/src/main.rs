// use std::env;
use std::fs::{self, File};
use std::io::{self, BufReader, prelude::*};
use std::path::Path;
use structopt::StructOpt;

// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    if let Ok(lines) = read_line(&args.path) {
        for line in lines {
            if let Ok(_line) = line {
                if _line.contains(&args.pattern) {
                    println!("{:?}", _line);
                }
            }
        }
    }
}

/// @see: https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
fn read_line<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// fn main() {
//     let args = Cli::from_args();
//     let mut content = fs::read_to_string(&args.path).expect("could not read file");

//     for line in content.lines() {
//         if line.contains(&args.pattern) {
//             println!("{}", line);
//         }
//     }
// }
