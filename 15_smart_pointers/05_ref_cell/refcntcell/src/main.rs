use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum List{
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil
}

use List::{Cons, Nil};

fn main() {
    let value = Rc::new(RefCell::new(5)); // "value" declared as immutable
    // wrapping an "i32" in "RefCell" which is wrapped inside "Rc" hence "5" has multiple owners who can mutate it
    // Though it can have mutable references from different variables, they all cannot exist at the sametime within a given scope
    // just because it is possible to compile a code that has multiple mutable references within a given scope doesn't mean it is possible to have them
    // the run time check will cause code crash by panic if there are multiple mutable references within a given scope
    // hence, we still need to make sure we have only one mutable reference within a given scope

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10; // "Rc" implements "Deref" trait and dereferencing it will give "RcCell" which allows you to borrow mutably an immutable data

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
