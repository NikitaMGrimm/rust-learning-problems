use std::error::Error;
use std::{env, fs};

pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
    pub case_sensitive: bool,
}

impl<'a> Config<'a> {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() <= 2 {
            return Err("Not enough arguments!");
        }

        let mut case_sensitive = env::var("CASE_SENSITIVE").is_ok();
        if args.len() > 3 && &args[3] == "CASE_SENSITIVE" {
            case_sensitive = true;
        }

        Ok(Config {
            query: &args[1],
            file_path: &args[2],
            case_sensitive,
        })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let result = match config.case_sensitive {
        true => search(config.query, &contents),
        false => search_case_insensitive(config.query, &contents),
    };

    for line in result {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let clean_query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&clean_query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_zero_result() {
        let query = "should not be in the content";
        let contents = "Hello World\nNot included!";

        assert_eq!(Vec::<&str>::new(), search(query, contents));
    }

    #[test]
    fn search_one_result() {
        let query = "duct"; // duct is in productive
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn search_two_result() {
        let query = "e"; // is in lines two and three
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive.", "Pick three."],
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
