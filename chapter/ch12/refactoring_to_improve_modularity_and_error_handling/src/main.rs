use std::{ env, process };
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let conf = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = minigrep::run(conf) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

// use std::{ env, fs, process, error };
// use error::Error;

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     let conf = Config::new(&args).unwrap_or_else(|err| {
//         println!("Problem parsing arguments: {}", err);
//         process::exit(1);
//     });

//     println!("Searching for {}", conf.query);
//     println!("In file {}", conf.filename);

//     if let Err(e) = run(conf) {
//         println!("Application error: {}", e);
//         process::exit(1);
//     };
// }

// fn run(conf: Config) -> Result<(), Box<dyn Error>> {
//     let contents = fs::read_to_string(conf.filename)?;
//         // .expect("Something went wrong reading the file");

//     println!("With text:\n***\n{}\n***", contents);

//     Ok(())
// }


// #[derive(Debug)]
// struct Config {
//     query: String,
//     filename: String,
// }

// // fn parse_config(args: &[String]) -> Config {
// //     let query = args[1].clone();
// //     let filename = args[2].clone();

// //     Config { query, filename }
// // }

// impl Config {
//     fn new(args: &[String]) -> Result<Config, &'static str> {
//         if args.len() < 3 {
//             // panic!("not enough arguments");
//             return Err("not enough arguments");
//         }
//         let query = args[1].clone();
//         let filename = args[2].clone();

//         // Config { query, filename }
//         Ok(Config { query, filename })
//     }
// }