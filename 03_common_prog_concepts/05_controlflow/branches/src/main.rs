fn main() {
    let number = 3;

    // if as statement:if{...;}
    if number % 4 == 0 {
        println!("number is divisible by 4"); // semicolon inside block makes if statement(statement doesn't return any value)
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    /*
        if number { // unlike C this doesn't compile
            println!("condition was true");
        }
    */

    // if as expression:if{...}
    let condition = true;
    let number = if condition {
        5 // no semicolon inside block makes if an expression(expression returns a value)
    } else {
        6
    };
    /*
        // This doesn't compile
        let number = if condition { // for this to compile both blocks should evaluate to value of same data type
            5
        } else {
            6.7
        }
    */

    println!("The value of number is: {}", number);
}
