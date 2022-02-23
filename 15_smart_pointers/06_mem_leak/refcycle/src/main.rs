use std::cell::RefCell;
use std::rc::Rc;
use crate::List::Cons;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>), // now we have the ability to change the list, meaning ability to change even "i32"
    // previously we had "Cons(Rc<RefCell<i32>>, Rc<List>)" meaning we had ability to change just the "i32"
    Nil
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> { // lets us access the 2nd element in "Cons"
        match self {
            List::Cons(_, item) => Some(item), // returns a reference to the list
            List::Nil => None
        }
    }
}

fn main() {
    let a = Rc::new(List::Cons(5, RefCell::new(Rc::new(List::Nil))));
    /*
          +----+-----+
    a --> |  5 |  Nil|
          +----+-----+
    */

    println!("a initial rc count = {}", Rc::strong_count(&a)); // 1
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(List::Cons(10, RefCell::new(Rc::clone(&a))));
    /*
          +----+-----+
    b --> | 10 |  a  |
          +----+-----+
    */

    println!("a rc count after b creation = {}", Rc::strong_count(&a)); // 2
    println!("b initial rc count = {}", Rc::strong_count(&b)); // 1
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() { // method is used to get a reference to list
        *link.borrow_mut() = Rc::clone(&b); // the reference is used to change "a" to have "b" as its tail
    }
    /*
          +----+-----+
    a --> |  5 |  b  |
          +----+-----+
    */

    println!("b rc count after changing a = {}", Rc::strong_count(&b)); // 2
    println!("a rc count after changing a = {}", Rc::strong_count(&a)); // 2
    /*
          +----+-----+
    a --> |  5 |  b  |
          +----+-----+
          +----+-----+
    b --> | 10 |  a  |
          +----+-----+
    // a cycle is created
    */
    /*
    println!("a next item = {:?}", a.tail()); // cause code crash
    // operator ":?" will try to walk until the end where it encounters "Nil"
    // cyclic reference causes it to continuously loop causing stack overflow to occur and hence code crash
    */
} // "b" was created at the last and hence it is on top of the stack and goes out of scope first executing its "drop" method
// when "drop" method is executed, its rc count is decremented from 2 to 1, hence it is not cleared from memory
// similar thing happens with "a" as well
// though the variable is out of scope the data is still present on the heap, i.e. memory leak
