
/*
enum List{
    Cons(i32, List), // throws compilation error
    Nil // explicitly defined for base case of recursion, not to be confused with "null" keyword used in case of invalid/absent value
}
// "Cons" can nest infinitely, hence compiler does not know the size of value of "List" type at compile time
// Hence, the compilation error
*/

enum List{
    Cons(i32, Box<List>), // now by putting the base element inside a box, we work around the problem mentioned above
    // now compiler knows that the 2nd element in the pair is a pointer and it can make a fixed space for the pointer to load it on the stack
    // note that the pointer size doesn't change based on the type of the data it is pointing to
    // It depends on the width of the address bus of the processor architecture
    Nil
}
// without "Box" we were trying to recursively put values inside each other
// now with "Box" we are effectively placing values next to each other

fn main() {
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
    println!("Hello, world!");
}
