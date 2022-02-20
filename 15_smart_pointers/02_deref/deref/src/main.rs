fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(x, 5);
    /*
    assert_eq!(y, 5); // throws compilation error
    // cannot compare an address and a value
    */
    assert_eq!(*y, 5); // as "y" is a reference, it needs to be dereferenced first in order to compare it with a number

    let b = Box::new(x); // a copy of "x" is placed on the heap and "b" is an address to that
    /*
    assert_eq!(b, 5); // throws compilation error
    // cannot compare a box(pointer) and a value
    */
    assert_eq!(*b, 5); // dereferencing a "Box<T>" will give "T"
}
