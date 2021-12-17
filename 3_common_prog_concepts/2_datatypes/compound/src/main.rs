fn main() {
    let tup = (500, 6.4, 1);

    let (_, y, _) = tup; // using pattern to unpack tuple aka destructuring

    let z = tup.2;
    println!("The 0th value of tuple is: {}", tup.0);
    println!("The 1st value of tuple is: {}", y);
    println!("The 2nd value of tuple is: {}", z);


    let a = [1, 2, 3, 4, 5]; // inferred data type [i32; 5], meaning 5 i32 values
    let zeroth = a[0];
    println!("The 0th value of array a is: {}", zeroth);

    let b = [3;5]; // create array b with 5 values all of which are 3
    let first = b[1];
    println!("The 1st value of array b is: {}", first);
}
