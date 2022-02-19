//! # **doc** crate
//! **doc** crate is an example to show how rust's documentation comments can be used

/// Adds one to the number given
///
/// # Examples
/// ```
/// let arg = 5;
/// let answer = doc::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

/*
//! # **doc** crate // doc comment code heading that is not related to any api
//! **doc** crate is an example to show how rust's documentation comments can be used // doc comment code heading that is not related to any api

/// Adds one to the number given // doc comments start right above the api that it documents
///
/// # Examples // section heading
/// ``` // code that demonstrates usage of function
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ``` // you can test this code block using "cargo test"
pub fn add_one(x: i32) -> i32 {
    x + 1
}
*/