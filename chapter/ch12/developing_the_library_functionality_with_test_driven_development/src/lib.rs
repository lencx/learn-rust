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

  // for line in search(&conf.query, &contents)
  // println!("**************************");
  // println!("{:#?}", search(&conf.query, &contents));

  println!("**************************");
  println!("[Searching ...]\n");

  for (i, item) in search(&conf.query, &contents).iter().enumerate() {
    println!("{}: {}", i, item);
  }

	Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn one_result() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";
    assert_eq!(
      vec!["safe, fast, productive."],
      search(query, contents)
    );
  }
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
  let mut result = vec![];
  for line in contents.lines() {
    if line.contains(query) {
      // println!("## {}: {}", linenumber, line);
      result.push(line);
    }
  }

  result
}