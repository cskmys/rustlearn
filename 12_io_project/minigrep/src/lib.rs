#[cfg(test)]
mod tests {
    use crate::{search_case_sensitive, search_case_insensitive};

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nDuct tape.";

        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}

use std::{fs, error, env};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        // now the user can enter "true" or "false" as the fourth argument to say whether search should be case sensitive or not
        // if the 4th argument is neither "true" or "false", then program will return an error
        // if the user doesn't provide any 4th argument, then directly check for the existence of "CASE_SENSITIVE" environment variable
        // we just ignore arguments after the 4th argument
        let case_sensitive = if args.len() >= 4 {
            args[3].clone().parse().or_else(|_|{
                Err("Invalid case-sensitivity argument")
            })
        } else {
            Ok(Config::does_case_sensitive_env_var_exist())
        }?;

        Ok(Config {query, filename, case_sensitive})
    }
    fn does_case_sensitive_env_var_exist() -> bool {
        env::var("CASE_SENSITIVE").is_ok()
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn error::Error>>{
    let contents = fs::read_to_string(config.filename)?;

    let res = if config.case_sensitive {
        search_case_sensitive(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in res {
        println!("{}", line);
    }

    Ok(())
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            res.push(line);
        }
    }
    res
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut res = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            res.push(line);
        }
    }
    res
}