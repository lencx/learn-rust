use std::{ env, fs, error };
use error::Error;

pub struct Config {
	query: String,
	filename: String,
	case_sensitive: bool,
}

impl Config {
	pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
		args.next();

		let query = match args.next() {
			Some(arg) => arg,
			None => return Err("Didn't get a query string"),
		};
		let filename = match args.next() {
			Some(arg) => arg,
			None => return Err("Didn't get a file name"),
		};

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

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	contents.lines()
		.filter(|line| line.contains(query))
		.collect()
}

pub fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	contents.lines()
		.filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
		.collect()
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
