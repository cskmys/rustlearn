use integtests::{add_two};
mod common;

#[test]
fn it_adds() {
    assert_eq!(common::get_nb(5), add_two(3));
}
