use std::error::Error;
use std::fs;

pub struct Config {
	query: String,
	filename: String,
}

impl Config {
	pub fn new(args: &[String]) -> Result<Config, &'static str> {
		if args.len() < 3 {
			return Err("not enough arguments");
		}
		let query = args[1].clone();
		let filename = args[2].clone();

		Ok(Config { query, filename })
	}
}

pub fn run(conf: Config) -> Result<(), Box<dyn Error>> {
	let contents = fs::read_to_string(conf.filename)?;

	println!("with text:\n{}", contents);

	Ok(())
}