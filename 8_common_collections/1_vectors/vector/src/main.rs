#[derive(Debug)]
enum Coin{
    Head,
    Tail
}

#[derive(Debug)]
enum SpreadsheetCell{
    Int(i32),
    Float(f64),
    Text(String)
}

fn main() {
    let v: Vec<i32> = Vec::new(); // as no values are inserted, type inference cannot work, hence need to annotate type here
    /*
    v.push(5); // throws compilation error
    // v is immutable
    */
    {
        let mut v1 = vec![1, 2, 3]; // "vec!" macro creates a vector and initializes them with values provided in [...]
        v1.push(5); // adding "5" at the end fo vector
        v1.push(6);
        v1.push(7);


        let ele = v1[2]; // "v1[2]" copied to "ele" as "i32" implements "copy" trait
        println!("v1[2] accessed by value as v1[2] is {} and accessed by a variable is {}", v1[2], ele);

    } // "v1"'s scope ends here and "drop" is called on "v1" here freeing its memory
    let v2 = vec![Coin::Head, Coin::Tail, Coin::Head];
    /*
    let ele = v2[2]; // throws compilation error
    // Coin is an enum which does not implement a Copy trait, hence doing this can move the ownership
    // in case of vectors it is weird to move ownership of just index 2 to "ele" while keeping the rest with "v2"
    // hence rust blocks it altogether with a compilation error
    // therefore, it is always better to use references to access a vector element at a particular index
    // even when "copy" trait is implemented, using references can save-time spent in copying
    */
    let ele = &v2[2];
    println!("v2[2] accessed by value as v1[2] is {:#?} and accessed by reference as &v2[2] is {:#?}", v2[2], ele);
    /*
    let tenth = &v2[9]; // throws runtime error
    // the index 9 is out of bounds
    */
    match v2.get(9) {
        Some(coin) => println!("v2[9] accessed by reference is {:#?}", coin),
        None => println!("Index 9 is out of bounds")
    };

    let mut v = vec![1, 2, 3];
    let ele = &v[0]; // when reference is used, immutable borrow occurs here
    println!("Element at v[0] is {}", ele);
    v.push(6); // "push" does "&mut self", hence a mutable borrow occurs here
    /*
    println!("Element at v[0] is {}", ele); // throws a compilation error
    // "v.push(6);" gave rise to a mutable borrow, hence any immutable borrows before that is invalidated
    // therefore, "ele" is no longer valid
    */
    print!("Elements of v are: ");
    for i in &v{ // using "&v" instead of "v" to avoid losing the ownership if "copy" trait was not implemented
        print!("{} ", i);
    }
    print!("\n");
    for i in &mut v{ // using "&mut v" a mutable borrow to change the elements of "v"
        *i = *i * 2;
    }

    print!("Elements of v after doubling them are: ");
    for i in &v{ // using "&v" instead of "v" to avoid losing the ownership if "copy" trait was not implemented
        print!("{} ", i);
    }
    print!("\n");

    let row= vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];
    println!("row in a spreadsheet is {:?}", row);
}
