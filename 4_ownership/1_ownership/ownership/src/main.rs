fn main() {
    let x = 1;
    let y = x;

    println!("x is {}, y(x's copy) is {}", x, y);

    makes_copy(y);
    println!("y is still accessible and it is {}", y);

    let z = makes_copy_ret(y);
    println!("z(returned as y's copy) is {} and y is still accessible and it is {}", z, y);



    let s1 = String::from("Hello");
    let s2 = s1;
    /*
        println!("{}, {}", s1, s2); // throws an error coz s1 has lost ownership
        // ownership issue arose with "String" but not "i32" coz
        // "i32" size is known at compilation time and it is stored on stack
        // so it is easy to make copies and let each variable have their own copy, hence no question of ownership.
        // but "String" size is not known at compilation time and their data is stored on heap
        // so assigning multiple pointer to the same heap address will open up room for errors like C/C++
        // hence, restricting the ownership to prevent any room for memory mismanagement errors
        // moreover, rust call "drop" automatically at every "}" hence, here a double free would happen if ownership is not restricted.
    */
    println!("s2(s1's move) is \"{}\"", s2);

    let s3 = s2.clone(); // clone makes deep copy and hence no ownership issues
    println!("s3(s2's deep copy) is \"{}\" and s2 still is \"{}\"", s3, s2);

    makes_move(s3);
    /*
        println!("s3 is \"{}\"", s3); // won't compile
        // the "move" operation done while passing s3 to function causes s3 to lose ownership
    */
    let s4 = makes_move_ret(s2);
    println!("s4(s2's move) is \"{}\"", s4);
    /*
        println!("s2 is \"{}\"", s2); // won't compile
        // the "move" operation done while passing s2 to function causes s2 to lose ownership
    */
}

fn makes_copy(zc:i32){
    println!("zc(y's copy) is {}", zc);
}

fn makes_copy_ret(zr:i32) -> i32{
    zr
}

fn makes_move(s4m: String){
    println!("s4m(s3's move) is \"{}\"", s4m);
}

fn makes_move_ret(s4r:String) -> String{
    s4r
}
