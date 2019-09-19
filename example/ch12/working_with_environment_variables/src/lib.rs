use std::{ env, fs, error };
use error::Error;

pub struct Config {
	query: String,
	filename: String,
  case_sensitive: bool,
}

impl Config {
	pub fn new(args: &[String]) -> Result<Config, &'static str> {
		if args.len() < 3 {
			return Err("not enough arguments");
		}
		let query = args[1].clone();
		let filename = args[2].clone();

    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

		Ok(Config { query, filename, case_sensitive })
	}
}

pub fn run(conf: Config) -> Result<(), Box<dyn Error>> {
	let contents = fs::read_to_string(conf.filename)?;

	// println!("with text:\n{}", contents);

  // for line in search(&conf.query, &contents)
  // println!("**************************");
  // println!("{:#?}", search(&conf.query, &contents));

  // println!("**************************");
  // println!("[Searching ...]\n");

  let result = if conf.case_sensitive {
    search(&conf.query, &contents)
  } else {
    search_case_insensitive(&conf.query, &contents)
  };

  for (i, item) in result.iter().enumerate() {
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

  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
    assert_eq!(
      vec!["safe, fast, productive."],
      search(query, contents)
    );
  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(
      vec!["Rust:", "Trust me."],
      search_case_insensitive(query, contents),
    )
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

pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
  // vec![]
  let query = query.to_lowercase();
  let mut result = Vec::new();

  for line in contents.lines() {
    if line.to_lowercase().contains(&query) {
      result.push(line);
    }
  }

  result
}