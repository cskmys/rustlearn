fn five() -> i32 {
    5
}

fn x_p_1(x: i32) -> i32{
    x + 1 // if you add a semi-colon here, the function can no longer return a value and a compilation error will occur
}

fn main() {
    /*
    let x = (let y = 6);
    // unlike other languages, here assignment doesn't return a value.
    // hence the code that uses *let* is a statement not an expression.
    */
    let y = {
        let x = 3;
        x_p_1(x) + 1 // expressions do not have ending semi-colons
    }; // statement end with semi-colon
    // here {...} evaluates to a value and hence it is an expression
    // let y = {...}; is a statement

    print_labeled_measurement(y, 'h');

    println!("function five returns {}", five());
}

fn print_labeled_measurement(value: i32, unit_label: char){ // in rust we use snake case instead of camel case to name functions
    println!("The measurement is: {}{}", value, unit_label);
}

