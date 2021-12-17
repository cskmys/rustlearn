use rand::Rng;
use std::io; // "use" includes "io" library part of "std" i,e, standard library to read input and write output // "Rng" is a trait that defines implementations of methods to generate random number

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    // "thread_rng()" generates a struct which contains the seed and random number generator
    // "gen_range(1..101)" function of the random number generator output by "thread_rng" use the seed to generate a random number in the range [1,100]

    println!("The secret number is: {}", secret_number); // added just for testing

    println!("Please input your guess.");

    let mut guess = String::new(); // creates mutable variable bound to an empty string
                                   // "let" is used to create a variable, "mut" indicates that the variable is mutable i,e, value can be changed
                                   // "String" is a string provided in "std" library which is growable
                                   // "new()" is a function that creates an empty string

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    // "stdin()" gives handler to the input stream of the program
    // "read_line(...)" is function of input stream which reads whatever comes on it as a string
    // "&" in "&mut guess" indicates that a reference to "guess" is passed, hence it can be inside the "read_line" without copying all of it's data
    // "read_line(...)" returns "io:Result" which is an enum type in rust which can have 2 options "ok" or "Err"
    // "io:Result" has an "expect" method which returns the value for "ok" and crashes the program and displays the message on "Err"

    println!("You guessed: {}", guess);
    // {} is a place holder for "guess"
}
