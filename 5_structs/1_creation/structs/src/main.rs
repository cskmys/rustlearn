struct User{ // defining a struct
    active: bool, // field of a struct
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32); // defining a tuple struct, a tuple with the behavior of struct

struct AlwaysEqual; // unit struct

/*
struct User{
    active: bool,
    username: String,
    email: &str, // throws compilation error
    sign_in_count: u64,
};
// without use of lifetimes, structs cannot hold references
*/

fn main() {

    let user1 = User{
        email: String::from("someone@example.com"), // order of fields is not important thanks to name of the fields
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    /*
    user1.email = String::from("anotheremail@example.com"); // throws compilation error as user1 is immutable
    */
    println!("User1's email is {}", user1.email);

    let mut user2 = build_user(String::from("someone@example.com"), String::from("someusername123"));
    user2.username = String::from("someusername1234"); // even to change just one field, whole struct should be mutable
    println!("User2's username is {}", user2.username);

    let user3 = User{
        email: user1.email, // order of fields is not important thanks to name of the fields
        username: String::from("someone"),
        active: user2.active,
        sign_in_count: 1,
    };
    println!("User3's status is {}", user3.active);

    let user4 = User{
        username: String::from("someone1234"),
        ..user3 // copies all the remaining fields from "user3", if you are using this shorthand, "user3" has to be at the last
    };
    println!("User4's has signed-in {} time(s)", user4.sign_in_count);

    /*
    // using "let <var_name> = <struct_name>{<another_var>};" is equivalent to using the "=" operator.
    // Hence, rules related to assignment apply here.
    // therefore, field with complex data types gets moved and fields of primitive data types get copied
    println!("User1's email is {}", user1.email); // throws compilation error
    // "email" field is a "String" type hence it will be moved
    // user1's email was moved to user3's email during "let user3 = User{ email: user1.email," was written
    println!("User3's email is {}", user3.email); // throws compilation error
    // user3's email was moved to user4 during "let user4 = User{ username: String::from("someone1234"), ..user3};"
     */
    println!("User4's status(copied) is {}", user4.active);
    // "active" field is of boolean type, hence it gets copied rather than being moved
    // "user3.active" was copied to "user4.active" during "let user4 = User{ username: String::from("someone1234"), ..user3};"
    println!("User3's username(not moved) is {}", user3.username);

    let black = Color(0, 0, 0);
    println!("black is ({}, {}, {})", black.0, black.1, black.2); // tuple struct accessed like normal structs

    let subject = AlwaysEqual; // now subject will need to abide by traits of "AlwaysEqual" though it does not hold any data
}

fn build_user(email: String, username: String) -> User {
    User{
        email: email,
        username, // shorthand for "username: username,"
        active: true,
        sign_in_count: 1,
    }
}