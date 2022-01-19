#[cfg(test)]
mod tests {
    use crate::{add_two};

    #[test]
    fn eq_test() {
        let result = add_two(setup_ip());
        assert_eq!(result, 4);
    }
    fn setup_ip() -> i32 {
        2
    }
    #[test]
    fn ne_test() {
        assert_ne!(add_two(setup_ip()), 5);
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}
