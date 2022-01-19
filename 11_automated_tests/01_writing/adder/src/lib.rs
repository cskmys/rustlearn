#[cfg(test)]
mod tests {
    use crate::{add_two, Guess, just_hello, Rect}; // path added to bring it into scope

    #[test] // "test" attribute added above a function to convert it into a test function which gets run on "cargo test" command
    fn eq_test() {
        let result = add_two(setup_ip()); // can call other non-test functions from within the test function
        assert_eq!(result, 4); // "assert_eq!" checks if "result" equals "4", if not it panics and prints both the parameters
        // in rust's "assert_eq!" and similar marcos, the order of parameters do not matter, "assert_eq!(result, 4);" and "assert_eq!(4, result);" are the same
    }
    fn setup_ip() -> i32 { // absence of "test" attribute shows that it is not a test function, rather it is a helper to setup the test
        2
    }
    #[test]
    fn ne_test() {
        assert_ne!(add_two(setup_ip()), 5); // "assert_ne!" checks if "result" is not equal to "5", if not it panics and prints both the parameters
    }
    // the "assert_eq!" and "assert_ne!" macros internally use "==" and "!=" respectively and also prints the values when the condition fails
    // hence the parameters must implement "PartialEq" and "Debug" traits
    // all primitive types and standard library types implement this
    // for user-defined structs and enums this can be derived using "#[derive(PartialEq, Debug)]" unless you need some custom comparison or display
    #[test]
    fn panic_test(){
        panic!("make this test fail"); // test fails
    }
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rect {
            w: 8,
            h: 7
        };
        let smaller = Rect {
            w: 5,
            h: 1
        };
        assert!(larger.can_hold(&smaller)); // "asert!" checks if input is "true" i.e. will a rect be able to hold another rect?, otherwise it panics
    }
    #[test]
    fn smaller_can_hold_larger() {
        let larger = Rect {
            w: 8,
            h: 7
        };
        let smaller = Rect {
            w: 5,
            h: 1
        };
        assert!(!smaller.can_hold(&larger)); // using "!" operator allows to check the opposite of previous test i.e. will a rect not be able to hold another rect?
    }
    #[test]
    fn just_hello_contains_name(){
        let res = just_hello();
        let name = "Carol";
        assert!(res.contains(name), "\"{}\" doesn't contain \"{}\"", res, name); // ""\"{}\" doesn't contain \"{}\"", res, name" is a format string that will be displayed on failure
    }
    #[test]
    #[should_panic]
    fn guess_more_than_100(){
        Guess::new(200);
    }
    #[test]
    #[should_panic]
    fn guess_more_than_100_bug(){
        Guess::new_with_bug(200);
    }
    #[test]
    #[should_panic(expected = "greater than 0")] // in "expected" argument a unique substring of expected panic message is entered
    fn guess_more_than_100_precise_fail(){
        Guess::new_precise(200);
    }
    #[test]
    #[should_panic(expected = "lesser than 101")] // if panic happened at the expected location, the panic message matches the substring in the "expected" argument, and "should_panic" test passes, otherwise it fails
    fn guess_more_than_100_precise_pass(){
        Guess::new_precise(200);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 != 4"))
        }
    }
    #[test]
    fn it_doesnt_work() -> Result<(), String> {
        if 2 + 2 == 5 {
            Ok(())
        } else {
            Err(String::from("2 + 2 != 5"))
        }
    }
}

#[derive(Debug)]
struct Rect{ // not "pub" hence it cannot be imported outside this module
    w: u32,
    h: u32
}

impl Rect{
    fn can_hold(&self, other: &Rect) -> bool {
        self.w > other.w && self.h > other.h
    }
}

pub fn add_two(a: i32) -> i32 { // making it "pub" makes it possible to be imported anywhere
    a + 2
}

pub fn just_hello() -> String{
    format!("Hello")
}

pub struct Guess {
    val: i32
}

impl Guess{
    pub fn new(val: i32) -> Guess{
        if val < 1 || val > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", val);
        }
        Guess{ val }
    }
    pub fn new_with_bug(val: i32) -> Guess{
        if val < 1 {
            panic!("Guess value must be between 1 and 100, got {}.", val);
        }
        Guess{ val }
    }
    pub fn new_precise(val: i32) -> Guess{
        if val < 1 {
            panic!("Guess value must be greater than 0, got {}.", val);
        } else if val > 100  {
            panic!("Guess value must be lesser than 101, got {}.", val);
        }
        Guess{ val }
    }
}
