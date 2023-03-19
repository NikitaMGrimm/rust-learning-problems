use std::error::Error;
use std::fs;

pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
}

impl<'a> Config<'a> {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() <= 2 {
            return Err("Not enough arguments!");
        }
        Ok(Config {
            query: &args[1],
            file_path: &args[2],
        })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{contents}");
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

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
}
