/*
     +---+---+
b -->| 3 | \ |
     +---+---+
              \  +---+---+    +---+---+     +---+
          a ---->| 5 |  -|--->| 10|  -|---->|Nil|
              /  +---+---+    +---+---+     +---+
     +---+---+
c -->| 4 | / |
     +---+---+
*/

enum List {
    Cons(i32, Box<List>),
    Nil
}

enum ListR<'a> { // lifetime parameter is required as we are going to be using references
    ConsR(i32, Box<&'a ListR<'a>>), // to try to solve the previous problem we create a list which will hold reference to another list
    // this way multiple lists can share a list
    NilR
}

use std::rc::Rc; // "Rc" not in the prelude, hence need to import it

enum ListRc{
    ConsRc(i32, Rc<ListRc>),
    NilRc
}

fn main() {
    let a = List::Cons(5, Box::new(List::Cons(10, Box::new(List::Nil))));
    let b = List::Cons(3, Box::new(a)); // "a" is moved here
    /*
    let c = List::Cons(4,Box::new(a)); // throws compilation error
    // "a" was already moved, hence "a" has lost ownership and is no longer valid
    // hence it is not possible for both lists "b" and "c" to share list "a", only one can use it
    */

    let ar = ListR::ConsR(5, Box::new(&ListR::ConsR(10, Box::new(&ListR::NilR))));
    /*
    let ar = ListR::ConsR(5, Box::new(&ListR::ConsR(10, Box::new(&ListR::NilR)))); // throws compilation error
    let br = ListR::ConsR(3, Box::new(&ar)); // causes compilation error
    // at the end of this scope "Box::new(&ListR::ConsR(10, Box::new(&ListR::NilR)))" of "ar" gets freed while "br" is still borrowing "ar"
    // hence the lifetime requirement of "ListR" gets violated which expects "Box::new(&ListR::ConsR(10, Box::new(&ListR::NilR)))" to be valid at least as long as the validity of "br"
    let cr = ListR::ConsR(4, Box::new(&ar)); // causes compilation error too
    // same reasoning as "br"
    */

    let arc = Rc::new(ListRc::ConsRc(5, Rc::new(ListRc::ConsRc(10, Rc::new(ListRc::NilRc)))));
    let brc = ListRc::ConsRc(3, Rc::clone(&arc)); // "arc.clone()" can also be used instead of "Rc::clone(&arc)", using "Rc::clone" is just a convention
    // here "Rc::clone" doesn't make a deep copy like "clone" of most types do, it just increases the reference count
    // hence when looking for performance issues you need to clearly check the type of variable which is calling a "clone" function to know if it does deep copy or reference counting
    let crc = ListRc::ConsRc(4, Rc::clone(&arc));


    let xrc = Rc::new(ListRc::ConsRc(5, Rc::new(ListRc::ConsRc(10, Rc::new(ListRc::NilRc)))));
    println!("cnt after creating xrc = {}", Rc::strong_count(&xrc)); // "Rc::strong_count" returns the reference count
    let yrc = ListRc::ConsRc(3, Rc::clone(&xrc));
    println!("cnt after creating yrc = {}", Rc::strong_count(&xrc));
    {
        let zrc = ListRc::ConsRc(4, Rc::clone(&xrc));
        println!("cnt after creating zrc = {}", Rc::strong_count(&xrc));
    } // "Drop" trait of "zrc" automatically decreases the reference count
    println!("cnt after zrc goes out of scope = {}", Rc::strong_count(&xrc));
} // "yrc" goes out of scope here first, followed by "xrc"
// when "yrc" goes out of scope reference count gets decremented to 1 therefore memory is not cleared, but when "xrc" goes out of scope reference count gets decremented to 0, hence, the memory is cleared as well
