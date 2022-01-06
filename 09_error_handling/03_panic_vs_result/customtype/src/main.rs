use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        Guess{value}
    }

    pub fn value(&self) -> i32{
        self.value
    }
}

fn did_you_win(guess: &Guess, secret_number: &i32) -> bool { // now that we use custom data type, we are always sure that the value is always in valid bounds
// fn did_you_win(guess: i32) -> bool{ // can send any i32 number, hence validation logic might be required
    // if num < 1 || num > 100 { // whenever you pass number to a function, it is tedious to do all the validation all the time
    //     println!("Please type a number between 1 to 100!");
    //     return false;
    // }
    // match  guess.cmp(&secret_number){
    match guess.value.cmp(secret_number){
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            return true;
        }
    }
    false
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        // println!("Please input your guess.");
        println!("Please input your guess(a number between 1 to 100)."); // here guess should be [1, 100]

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = match guess.trim().parse(){
            Ok(num) => {
                // if num < 1 || num > 100 { // whenever you pass number to a function, it is tedious to do all the validation all the time
                //     println!("Please type a number between 1 to 100!");
                //     continue;
                // } else {
                //     num
                // }
                Guess::new(num) // elegant solution is to create a data type
            },
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        // println!("You guessed: {}", guess);
        println!("You guessed: {}", guess.value);

        if did_you_win(&guess, &secret_number){
            break;
        }
    }
}
