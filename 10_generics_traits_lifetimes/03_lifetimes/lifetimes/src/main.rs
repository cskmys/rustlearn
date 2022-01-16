/*
fn longest_str(x: &str, y: &str) -> &str{ // throws compilation error
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// function returns a reference and this reference is one of the input parameters.
// at compile time, compiler cannot infer which block in the "if-else" statement will be executed.
// hence, we don't know which reference will be returned
// therefore, borrow checker cannot check the validity of the returned reference.
// hence, compiler needs an annotation from the programmer to tell the borrow checker how to check for validity of returned reference
*/
fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str { // life time "'a" is declared as a generic parameter first before annotating parameters with it
    // the annotation conveys to compiler for some lifetime "'a" the function takes "x" and "y" and return a reference which will live at least as long as "'a"
    // hence, the compiler understands that the lifetime of returned reference is the same as the smaller lifetime among "x" and "y"
    // therefore, generic lifetime "'a" is substituted by a concrete lifetime which is the smaller lifetime among "x" and "y"
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn join_str<'a>(x: &'a mut String, y: &String) -> &'a String { // here we care about lifetime of only "x" coz it is returned in the end and hence will matter even after returning from the function
    // lifetime of "y" doesn't matter as it has no relationship with return type after returning from the function
    *x = format!("{}{}", x, y);
    x
}

/*
fn new_str<'a>() -> &'a String { // no point writing lifetime here coz return value lifetime is not related to parameter lifetime
    let s = String::from("new string");
    let s_ref = &s;
     s_ref // throws compilation error
} // "s" goes out of scope here, hence "s_ref" is a dangling reference
// returning dangling reference "s_ref"
// the only solution is to return "String" instead of "&String"
*/

fn main() {
    /*
    {
        let r;                                   //----------+-- 'a
        {                                        //          |
            let x = 5;                           //--+-- 'b  |
            r = &x; // throws compilation error  //  |       |
        }                                        //--+       |
        println!("r: {}", r);                    //----------+
    }
    // the scope/lifetime of "r" is marked using "'a" and of "x" by "'b"
    // from the figure we can see that "'a" and "'b" doesn't overlap where "r" is used
    // this causes "r" to be a dangling reference
    */
    {
        let r;                    //----------+-- 'a
                                        //          |
        let x = 5;                 //--+-- 'b  |
        r = &x;                         //  |       |
                                        //  |       |
        println!("r: {}", r);           //--+-------+
        // "'a" and "'b" overlaps wherever "r" is used
        // hence is valid
    }

    let str1 = String::from("Hello, world!");
    let str2 = String::from("xyz");

    let mut str1a = str1.clone();
    let str2a = str2.clone();
    let resa = longest_str(str1a.as_str(), str2a.as_str()); // "str1a" and "str2a" has (almost) the same lifetime
    println!("The longest string(a): \"{}\"", resa);
    let resa = join_str(&mut str1a, &str2a);
    println!("The joined string(a): \"{}\"", resa);

    let mut str1b = str1.clone();
    {
        let str2b = str2.clone();

        let resb = longest_str(str1b.as_str(), str2b.as_str()); // now "str2b" has the shorter lifetime, hence the concrete lifetime substituted will be the lifetime of "str2b"
        // therefore, the lifetime of returned reference "resb" is same as lifetime of "str2b"
        println!("The longest string(b): \"{}\"", resb); // "str2b" is still valid therefore "resb" is valid as well
        let resb = join_str(&mut str1b, &str2b); // depends only on lifetime of "str1b"
        println!("The joined string(b): \"{}\"", resb); // "str1b" is still valid and hence, "resb" is valid as well
    }

    /*
    let mut str1c = str1.clone();
    let resc1;
    let resc2;
    {
        let str2c = str2.clone();
        resc1 = longest_str(str1c.as_str(), str2c.as_str()); // now "str2c" has the shorter lifetime, hence the concrete lifetime substituted will be the lifetime of "str2c"
        // therefore, the lifetime of returned reference "resc1" is same as lifetime of "str2c"
        resc2 = join_str(&mut str1c, &str2c);
    } // "str2c" is no longer valid, therefore, "resc1" is invalid too
    // println!("The longest string: \"{}\"", resc1); // throws compilation error
    println!("The joined string: \"{}\"", resc2);
    // "resc1" is used after it has become invalid
    */

    let mut str1c = str1.clone();
    let resc2;
    {
        let str2c = str2.clone();
        resc2 = join_str(&mut str1c, &str2c); // depends only on lifetime of "str1c"
    }
    println!("The joined string(c): \"{}\"", resc2); // "str1c" is still valid and hence, "resc2" is valid as well

    let str2 = "xyz"; // declaring "str2" as "str" instead of "String" and this changes everything
    let str1d = str1.clone();
    let resd;
    {
        let str2d = str2.clone(); // apparently "clone" is not creating a new copy of "str2"
        resd = longest_str(str1d.as_str(), str2d); // as new copy is not created "str2d" has the same lifetime as "str2", hence "str2d" and "str1d" has (almost) same lifetime
        // therefore, the lifetime of returned reference "resd" is same as lifetime of "str1d" and "str2d"
    } // "str2d" doesn't become invalid, therefore, "resd" doesn't become invalid too
    println!("The longest string(d): \"{}\"", resd); // "resd" is valid
}
