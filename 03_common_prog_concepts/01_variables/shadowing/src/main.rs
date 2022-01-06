fn main() {
    let z = 5;

    let z = z + 1; // 5 is lost from shadowing

    {
        let z = z * 2; // inner "z" takes latest outer "z" for computation and shadows outer "z"
        println!("The value of z in the inner scope is: {}", z); // inner "z" is printed as it shadows outer "z"
    } // end of scope leads to end of shadowing by inner "z"

    println!("The value of z is: {}", z); // latest "z" is printed

    let spaces = "   "; // "spaces" is string data type
    let spaces = spaces.len(); // now "spaces" is no longer a string but a length
    println!("There are {} spaces", spaces);

    /*
    let mut spaces = "   "; // "spaces" is string data type
    spaces = spaces.len(); // this is assignment(using the same variable), not shadowing(creating new variable with same name)
    // hence the data type difference will prevent code from compiling as we cannot mutate a variable data type
    */
}
