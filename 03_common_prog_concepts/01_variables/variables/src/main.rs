const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // constant
// always immutable, data-type has to be specified and can be of any scope
// can be assigned to a constant expression only, not an expression computed at runtime

// let y = 7; variables(even immutable) cannot be have a global scope

fn main() {
    println!("The number of seconds in three hours is {}", THREE_HOURS_IN_SECONDS);
    /*
    let x = 5;
    println!("The value of x is {}", x);

    x = 6; // throws a compilation error cause by default x is immutable i,e, cannot be assigned to more than once
    println!("The value of x is {}", x);
    */
    let mut x = 5; // mut conveys that the value can be changed from anywhere in the program
    println!("The value of x is {}", x);

    x = 6; // throws a compilation error cause by default x is immutable i,e, cannot be assigned to more than once
    println!("The value of x is {}", x);

    let y = x + 1; // immutable can be assigned as a result of an expression having mutable variable during runtime
    println!("The value of y is {}", y);
}
