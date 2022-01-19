use integtests::{add_two};
mod common;

#[test]
fn it_adds_two() {
    assert_eq!(common::get_nb(4), add_two(2));
}
