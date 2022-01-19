use std::{env, fs, process, error};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {}", err);
        process::exit(1); // will exit the program immediately and returns the argument number to OS as exit status code
    });

    println!("Searching for \"{}\"", config.query);
    println!("In file \"{}\"", config.filename);

    if let Err(e) = run(config){
        println!("Application error: {}", e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn error::Error>>{ // previously this function returned nothing in other words, it returned "()", so return type is "Return<(), ...>"
    // "Box<dyn Error>" means a value of type that implements the "Error" trait
    // this allows us to return error values of different datatype under different error cases
    let contents = fs::read_to_string(config.filename)?; // upon an error, "?" operator returns from the function with an error value
    println!("With text:\n{}", contents);
    Ok(()) // returning "()" meaning nothing upon success requires wrapping with "Ok" as the return is "Result" type
}

struct Config{
    query: String,
    filename: String
}

impl Config{
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {query, filename})
    }
}