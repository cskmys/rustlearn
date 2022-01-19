#[cfg(test)]
mod tests {
    use crate::search;

    #[test]
    fn one_result() { // building a test case
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents)); // "search" is a function that you'd like to build via TDD
        // it takes a query and contents of a file as a "&str" as input and return the vector of lines where the string is present as output
    }
}

use std::{fs, error};

pub struct Config {
    pub query: String,
    pub filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {query, filename})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn error::Error>>{
    let contents = fs::read_to_string(config.filename)?;
    println!("With text:\n{}", contents);
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> { // lifetime applied to "contents" and return as "contents" is the parameter that is connected to the output
    // hence, "contents" should be alive at least as long as the output
    vec![]
}