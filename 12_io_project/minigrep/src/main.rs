use std::env; // to use "std::env::args" to read command line arguments
// we import "std::env" in place of "std::env::args" coz writing "env::args()" is more clear than just going "args()"
use std::fs; // "std::fs" for file handling

fn main() {
    let args: Vec<String> = env::args().collect(); // "env::args()" will panic if unicode values are entered in command line
    // for taking unicode input use "std::env::args_os", which returns "OsString", whose values differ based on the OS, instead of "String"
    // for sake of simplicity we choose to go with "env::args()"
    // "collect" turns the iterator into a vector of all the values produced by the iterator
    // "collect" can return many types of data, hence we need to annotate "args" to specify what type the return of "collect" should be

    println!("{:?}", args); // ":?" to use the debug formatter
    // the 0th argument is the name of the program followed by the strings(separated by a whitespace) that were entered on the terminal
    let query = &args[1];
    let filename = &args[2];
    // error handling is needed for cases when user provides less than 2 arguments

    println!("Searching for \"{}\"", query);
    println!("In file \"{}\"", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    // "read_to_string" takes in a filename, opens the file, reads its contents and returns them as "Result<String>"
    // upon success you'll get "Ok(String)" and on failure you'll get "Err(String)"
    println!("With text:\n{}", contents);
}
