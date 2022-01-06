fn main() {
    let s1 = String::from("salut");
    /*
    let h1 = &s1[0]; // throws compilation error
    // in rust you cannot index strings like you index vectors
    */
    let h1 = &s1[0..1];
    println!("The 0th character of string \"{}\" obtained by slicing is {}", s1, h1);
    print!("The characters of the string \"{}\" obtained by iteration are: ", s1);
    for c in s1.chars(){
        print!("{} ", c);
    }
    print!("\n");
    print!("The bytes of the string \"{}\" obtained by iteration are: ", s1);
    for b in s1.bytes(){
        print!("{} ", b);
    }
    print!("\n");

    let s2 = String::from("Здравствуйте");
    /*
    let h2 = &s2[0..1]; // throws runtime error
    // each rust character is 2-bytes for these unicode characters, hence slicing anywhere other than the boundary of character is not allowed
    */
    let h2 = &s2[0..2];
    println!("The 0th character of string \"{}\" obtained by slicing is {}", s2, h2);
    print!("The characters of the string \"{}\" obtained by iteration are: ", s2);
    for c in s2.chars(){
        print!("{} ", c);
    }
    print!("\n");
    print!("The bytes of the string \"{}\" obtained by iteration are: ", s2);
    for b in s2.bytes(){
        print!("{} ", b);
    }
    print!("\n");

    let s3 = String::from("नमस्ते");
    /*
    let h3 = &s3[0..2]; // throws runtime error
    // each rust character is 3-bytes for these unicode characters, hence slicing anywhere other than the boundary of character is not allowed
    */
    let h3 = &s3[0..3];
    println!("The 0th character of string \"{}\" obtained by slicing is {}", s3, h3);
    print!("The characters of the string \"{}\" obtained by iteration are: ", s3);
    for c in s3.chars(){
        print!("{} ", c);
    }
    print!("\n");
    print!("The bytes of the string \"{}\" obtained by iteration are: ", s3);
    for b in s3.bytes(){
        print!("{} ", b);
    }
    print!("\n");
}
