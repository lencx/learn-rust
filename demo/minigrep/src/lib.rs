use std::{ env, fs, error };
use error::Error;

pub struct Config {
	query: String,
	filename: String,
	case_sensitive: bool,
}

impl Config {
	pub fn new(args: &[String]) -> Result<Config, &'static str> {
		if args.len() <  3 {
			return Err("not enough arguments")
		}
		let query = args[1].clone();
		let filename = args[2].clone();
		let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

		Ok(Config { query, filename, case_sensitive })
	}
}

pub fn run(conf: Config) -> Result<(), Box<dyn Error>> {
	let contents = fs::read_to_string(conf.filename)?;
	let result = if conf.case_sensitive {
		search(&conf.query, &contents)
	} else {
		search_insensitive(&conf.query, &contents)
	};

	for item in result {
		println!("{}", item)
	}

	Ok(())
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
	let mut result = Vec::new();
	for line in contents.lines() {
		if line.contains(query) {
			result.push(line);
		}
	}

	result
}

pub fn search_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
	let mut result = Vec::new();
	let query = query.to_lowercase();

	for line in contents.lines() {
		if line.to_lowercase().contains(&query) {
			result.push(line);
		}
	}

	result
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn search_content() {
		let query = "Rust's";
		let contents = "\
Rust's speed, safety, single binary output,
and cross-plaform support make it an ideal language for creating command line tools,
so for our project, ...";

		assert_eq!(vec!["Rust's speed, safety, single binary output,"], search(&query, &contents));
	}

	#[test]
	fn search_content_insensitive() {
		let query = "PrOj";
		let contents = "\
Rust's speed, safety, single binary output,
and cross-plaform support make it an ideal language for creating command line tools,
so for our project, ...";

		assert_eq!(vec!["so for our project, ..."], search_insensitive(&query, &contents));
	}
}
