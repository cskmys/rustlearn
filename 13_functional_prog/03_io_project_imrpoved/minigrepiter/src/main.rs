use std::{env, process};
use minigrepiter::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();

    // let config = Config::new(&args).unwrap_or_else(|err|{
    let config = Config::new(env::args()).unwrap_or_else(|err|{ // directly passing the iterator "env::Args" instead of "Vec<String>" like before
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrepiter::run(config){
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}