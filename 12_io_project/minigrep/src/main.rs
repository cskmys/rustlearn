use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for \"{}\"", config.query);
    println!("In file \"{}\"", config.filename);

    let contents = fs::read_to_string(config.filename).expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);
}

struct Config{
    query: String,
    filename: String
}

fn parse_config(args: &[String]) -> Config { // "args" is owned by "main", and is borrowed by "parse_config"
    let query = args[1].clone(); // hence if a variable in "parse_config" tries to take ownership, it would violate the borrowing rules
    let filename = args[2].clone(); // therefore, we are cloning to take full ownership
    // though this will cost a little in time n space, it makes code very simple and straight forward
    // in this case, we are trading a little bit of performance for simplicity

    Config {query, filename}
}