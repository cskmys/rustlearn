#[cfg(test)]
mod tests { // by default, threads are used to all the tests are run in parallel. to run tests sequentially set number of threads to 1 using "cargo test -- --test-threads=1"
    use crate::{add_two, prints_and_returns_10};

    #[test]
    #[ignore]
    fn this_test_will_pass() { // this test will be excluded as "ignore" is used above it
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }

    #[test]
    fn add_two_and_two() { // to test this and "add_three_and_two" while ignoring the rest, execute "cargo test add"
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() { // to test only this function execute "cargo test one_hundred"
        assert_eq!(102, add_two(100));
    }
}

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a); // this is displayed only if the test fails, to display always use "cargo test -- --show-output"
    10
}

pub fn add_two(a: i32) -> i32{
    a + 2
}