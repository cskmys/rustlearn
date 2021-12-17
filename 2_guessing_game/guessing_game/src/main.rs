use rand::Rng; // to include "Rng" which is a trait that defines implementations of methods to generate random number
use std::cmp::Ordering; // to include the enum "Ordering" which defines the 3 possible outcomes(<, =, >) when 2 comparable quantities are compared
use std::io; // "use" includes "io" library part of "std" i,e, standard library to read input and write output

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    // "thread_rng()" generates a struct which contains the seed and random number generator
    // "gen_range(1..101)" function of the random number generator output by "thread_rng" use the seed to generate a random number in the range [1,100]

    println!("The secret number is: {}", secret_number); // added just for testing

    loop {
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

        let guess: u32 = guess.trim().parse().expect("Please type a number!"); // converts string to 32-bit unsigned integer
        // shadowing the variable name allows you to reuse "guess" instead of having separate ones for string and integer
        // "String::trim()" will remove preceding and leading whitespaces, this is needed cause "read_line" would have read '\n' when the user pressed enter
        // "String::parse()" can parse a string into a variety of data-types, hence "u32" is explicitly specified
        // just like "read_line", "parse" return "Result" too, hence we have "except"

        println!("You guessed: {}", guess);
        // {} is a place holder for "guess"

        match  guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // exit the loop
            }
        }
        // "cmp" function returns an enum
        // because of "cmp", both "guess" and "secret_number" are assumed to be of the same data type.
        // "guess" has been explicitly mentioned as "u32" hence "secret_number" is automatically interpreted as u32 instead of it's default i32.
        /*
            match pattern_ret_func(){
                pattern1 => code to execute when pattern1 is output by pattern_ret_func,
                pattern2 => pattern2 code,
            }
        */
    }
    /*
        loop {
            some code
        }
        Executes the code within the braces forever until the program crashes or an exit code is reached
    */
}
