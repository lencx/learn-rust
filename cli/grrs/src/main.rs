use std::fs::{self, File};
use std::io::{self, BufReader, prelude::*};
use std::path::Path;
use structopt::StructOpt;

// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt, Debug)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    eprintln!("This is an error! :(");
    let args = Cli::from_args();
    println!("======= args =======");
    println!("{:#?}", args);
    // Option 1.
    let result = read_line(&args.path);
    match result {
        Ok(content) => {
            // println!("file content: {:?}", content);
            for (index, line) in content.enumerate() {
                if let Ok(_line) = line {
                    if _line.contains(&args.pattern) {
                        println!("======= match line [{}] =======", index);
                        println!("{}", _line);
                    }
                }
            }
        },
        Err(err) => println!("Oh noes: {}", err)
    }

    // read_content(&args.path);

    // // Option 2.
    // if let Ok(lines) = read_line(&args.path) {
    //     for (index, line) in lines.enumerate() {
    //         if let Ok(_line) = line {
    //             if _line.contains(&args.pattern) {
    //                 println!("======= match line [{}] =======", index);
    //                 println!("{}", _line);
    //             }
    //         }
    //     }
    // }
}

/// @see: https://doc.rust-lang.org/stable/rust-by-example/std_misc/file/read_lines.html
fn read_line<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// fn main() {
//     let args = Cli::from_args();
//     // let mut content = fs::read_to_string(&args.path)?;
//     let mut content = fs::read_to_string(&args.path).expect("could not read file");

//     for line in content.lines() {
//         if line.contains(&args.pattern) {
//             println!("{}", line);
//         }
//     }
// }

fn read_content<P>(filename: P) -> Result<(), Box<dyn std::error::Error>>
where P: AsRef<Path> {
    let result = fs::read_to_string(filename);
    let content = match result {
        Ok(content) => { content },
        Err(err) => { return Err(err.into()); },
    };
    // println!("file content: {}", content);
    Ok(())
}