fn main() {
    let some_number = Some(5); // type inferred as Option<i32> which means either it is "i32" or "None"
    let some_string = Some("a string"); // type inferred as Option<&str>
    /*
    let absent_number = None; // throws compilation error
    // cause the type cannot be inferred, "None" can be with any "T" in "Option<T>"
    */
    let absent_number : Option<i32> = None;
    let x = 5;
    /*
    let sum = x + some_number; // throws compilation here
    // "some_number" will hold a number when it is valid, otherwise it is "None"
    // Thanks to "Option<T>" type we avoid a situation wherein "None" gets added with a regular number
    // Hence, compiler forces us to perform a check to make sure there is a valid value in "some_number" and if there is convert "Option<i32>" to regular "i32" before using it with another regular "i32" number
    */
    let sum = x;
    if some_number != None{
        println!("sum is {}", sum + some_number.unwrap());
    } else {
        println!("No number or valid number to add")
    }
}
