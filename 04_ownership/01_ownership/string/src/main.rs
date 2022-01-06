fn main() {
    let s1 = "Hello, world!";
    println!("{}", s1);
    /*
        s1.remove(0); // "remove" method doesn't even exist as you cannot mutate a "str" data
    */

    let s2 = String::from(s1);
    println!("{}", s2);
    /*
        s2.remove(0); // throws a compilation error as s2 is immutable
    */

    let mut s3 = String::from(s1);
    s3.remove(0);
    println!("{}", s3);
    // with "String" you have an option to keep it mutable or immutable and if it's mutable, you can even modify it
}