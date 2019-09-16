use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 {
        let query = &args[1];
        let filename = &args[2];

        println!("args: {:?}", args);
        println!("args length: {:?}", args.len());
        println!("Searching for {}", query);
        println!("In file {}", filename);
    } else {
        println!("example: cargo run abc a.txt");
    }
}
