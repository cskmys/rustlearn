#[cfg(test)]
mod tests {
    use crate::search;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
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

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // iterate through each line of the "contents":
    //     check whether the lie contains our "query":
    //         if it does, add it to the list values to return
    //         else, do nothing
    // return the list of results that match
    let mut res = Vec::new();
    for line in contents.lines() { // "lines()" return an iterator hence you can do a "for" loop on it
        if line.contains(query){ // "contains" method does all the checking for us
            res.push(line);
        }
    }
    res
}