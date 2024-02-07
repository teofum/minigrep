use std::error::Error;
use std::{env, fs};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
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
            search_case_insensitive(query, contents)
        );
    }
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn from_args(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Query string missing"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("File path missing"),
        };

        let ignore_env = env::var("IGNORE_CASE").is_ok();
        let ignore_arg = match args.next() {
            Some(arg) => arg == "--ignore-case",
            None => false,
        };

        let ignore_case = ignore_arg || ignore_env;

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let lines: Vec<&str> = contents
        .lines()
        .filter(|line| line.contains(query))
        .collect();

    lines
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    let lines: Vec<&str> = contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect();

    lines
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}
