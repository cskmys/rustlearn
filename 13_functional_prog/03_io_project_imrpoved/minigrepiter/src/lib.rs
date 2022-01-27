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
    // pub fn new(args: &[String]) -> Result<Config, &str> {
    // logically there is no link between "args" and output, hence this should be "pub fn new(args: &[String]) -> Result<Config, &'static str> {"
    // but from compiler point of view, there could be a link between "&String" in input and "&str" in output, hence "args" should be alive at least the duration of output
    // so this should have been actually written as "pub fn new<'a>(args: &'a [String]) -> Result<Config, &'a str> {", but due to ellison rules, there was no need to explicitly write it
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> { // now takes in iterator instead of "String" slice, "mut" coz using "next" will change internal state
        // now that there is no reference in the input state, the output reference should be defined as "static"
        // if args.len() < 3 {
        //     return Err("not enough arguments");
        // }
        // let query = args[1].clone(); // previously cloning was done to cause args to not lose ownership
        // let filename = args[2].clone();
        // let case_sensitive = if args.len() >= 4 {
        //     args[3].clone().parse().or_else(|_|{
        //         Err("Invalid case-sensitivity argument")
        //     })
        // } else {
        //     Ok(Config::does_case_sensitive_env_var_exist())
        // }?;

        args.next(); // skipping the first argument which contains the name of the binary
        let query = match args.next(){
            Some(query) => query,
            None => return Err("not enough arguments")
        };
        let filename = match args.next(){
            Some(filename) => filename,
            None => return Err("not enough arguments")
        };
        let case_sensitive = match args.next(){
            Some(case_sensitive) => {
                case_sensitive.parse().or_else(|_|{Err("Invalid case-sensitivity argument")})?
            },
            None => Config::does_case_sensitive_env_var_exist()
        };

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

    // let res = config.case_sensitive.then(||search_case_sensitive(&config.query, &contents))// if "true" then closure is executed and it's output is wrapped in "Some" else "None"
    //     .unwrap_or_else(||search_case_insensitive(&config.query, &contents)); // if false

    // for line in res {
    //     println!("{}", line);
    // }
    res.iter().for_each(|line| {println!("{}", line);});

    Ok(())
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut res = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         res.push(line);
    //     }
    // }
    // res
    contents.lines().filter(|line| line.contains(query)).collect() // more concise and avoids use of intermediate mutable variable
    // functional programming minimizes the amount of mutable state and makes code more clear and concise
    // if there is no mutable state, it is easy to make it embarrassingly parallel
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let query = query.to_lowercase();
    // let mut res = Vec::new();
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         res.push(line);
    //     }
    // }
    // res
    contents.lines().filter(|line| line.to_lowercase().contains(query.to_lowercase().as_str())).collect()
}