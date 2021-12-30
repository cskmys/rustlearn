fn main() {
    let s1 = String::from("hello");
    println!("s1(an immutable string) is \"{}\"", s1);

    let (s2, len) = calc_len(s1);
    println!("s2(ret of s1's move) is \"{}\" and it's of length {}", s2, len);
    /*
        println!("s1 is {}", s1); // throws compilation error
        // s1's ownership was lost
    */

    // "& <any_variable>" is used to create a reference to a variable
    let len = calc_len_ref(&s2); // s2's reference was passed not s2 itself
    println!("s2(which still hasn't lost ownership) is \"{}\" and it's of length {}", s2, len);

    let mut s3 =  String::from("hahaha"); // creating a new string
    let len = calc_len_ref(&s3);
    println!("s3(a new mutable string) before deleting first character is \"{}\" and it's of length {}", s3, len);
    // let len = del0_calc_len_ref(&mut s2); // will throw error coz s2 is immutable
    let len = del0_calc_len_ref(&mut s3); // "&mut <mutable_variable>" is used to create a mutable reference to a mutable variable
    println!("s3(which still hasn't lost ownership) after deleting first character is \"{}\" and it's of length {}", s3, len);
}

fn calc_len(smr: String) -> (String, usize){
    let length = smr.len();
    (smr, length) // to regain the ownership of the parameter that was passed outside the function, it must be returned
    // which is bit annoying to do everytime
}

fn calc_len_ref(sr : &String) -> usize{ // sr is a reference
    /*
        sr.remove(0); // throws compilation error
        // by default references are immutable, hence you cannot use it to modify the data that it is referring to
    */
    sr.len() // as reference is used, ownership is not lost and hence it need to returned to facilitate gaining back of ownership
} // here sr, the reference goes out of scope, but it has no ownership of what it refers to, so when "drop" gets called nothing happens

fn del0_calc_len_ref(sr : &mut String) -> usize{ // sr is a mutable reference
    sr.remove(0);
    sr.len()
}