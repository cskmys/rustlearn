use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node { // node of a tree
nb: i32,
    children: RefCell<Vec<Rc<Node>>> // using "Rc<Node>" for a child node will allow its ownership to be shared
    // wrapping children in "RefCell" will allow modification of children nodes
    // sharing ownership of children with variables allow direct access to each node in the tree
} // you can go from a node to its children by doing "Node::children" but you cannot go from node to its parent as their is no reference to parent node

#[derive(Debug)]
struct Node2{
    nb: i32,
    parent: RefCell<Weak<Node2>>, // to manipulate parent node from a child node you'll child node to be aware of its parent and wrap it in "RefCell<T>"
    // "parent" cannot be "Rc<Node>" coz that would create a reference cycle where parent node would point to a child node via "children" field and
    // child node would point to parent node via "parent" field causing "strong_count" to never be 0 and hence causing a memory leak

    // a node should own its children but not its parent coz when a node is dropped we want its children to be dropped not its parent
    // in other words node should have strong ownership of its children and no ownership of its parent
    // hence node parent is connected by "Weak<Node>" and node child is connected by "Rc<Node>"
    children: RefCell<Vec<Rc<Node2>>>
}

fn main() {
    let leaf = Rc::new(Node{ // it is easier to build the tree bottom-up, so we first create leaves and then the rest
        nb: 3,
        children: RefCell::new(vec![])
    });
    let branch = Rc::new(Node{
        nb: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)])
    });
    println!("tree1: {:?}", branch);
    println!("tree1 leaves: {:?}\n", branch.children);

    // after creation of "Rc<T>" its "strong_count" is 1 and "weak_count" is 0
    // if you do "xyz = Rc::clone(&abc)" then "strong_count" of "abc" increments by 1 as "Rc::clone" is acting on "abc" to produce "Rc<T>"
    // if you do "xyz = Rc::downgrade(&abc)" then "weak_count" of "abc" increments by 1 as "Rc::downgrade" is acting on "abc" to produce "Weak<T>"
    let leaf2 = Rc::new(Node2 {
        nb: 3,
        parent: RefCell::new(Weak::new()), // initially we are just leaving the parent empty
        children: RefCell::new(vec![])
    }); // after creation, "strong_count" of "leaf2" is 1 and "weak_count" is 0
    println!("tree2 leaf(before creation of branch), strong = {}, weak = {}\n", Rc::strong_count(&leaf2), Rc::weak_count(&leaf2));
    // in general you don't know if the parent node exists or not, hence you check its presence using "Weak<T>::upgrade"
    println!("tree2 leaf parent(before creation of branch) = {:?}", leaf2.parent.borrow().upgrade()); // parent node not yet created, hence you'll get "None" as the answer
    {
        let branch2 = Rc::new(Node2{
            nb: 5,
            parent: RefCell::new(Weak::new()), // in this tree branch has no parent
            children: RefCell::new(vec![Rc::clone(&leaf2)]) // "strong_count" of "leaf2" increments by 1, while "weak_count" remains the same
        }); // after creation, "strong_count" of "branch2" is 1 and "weak_count" is 0

        println!("tree2 branch(after creation with leaf as it child, and before linking it as leaf's parent), strong = {}, weak = {}", Rc::strong_count(&branch2), Rc::weak_count(&branch2));
        println!("tree2 leaf(after linking leaf as child of branch, and before linking branch as its parent), strong = {}, weak = {}\n", Rc::strong_count(&leaf2), Rc::weak_count(&leaf2));

        *leaf2.parent.borrow_mut() = Rc::downgrade(&branch2); // to link the parent-child relationship between a "leaf" and its "branch"
        // the link needs to be a weak reference, hence we do "Rc::downgrade"
        // then "weak_count" of "branch2" increases by 1, while "strong_count" remains the same
        println!("tree2 branch(after linking it as parent of leaf), strong = {}, weak = {}", Rc::strong_count(&branch2), Rc::weak_count(&branch2));
        println!("tree2 leaf(after linking branch as its parent), strong = {}, weak = {}\n", Rc::strong_count(&leaf2), Rc::weak_count(&leaf2));

        println!("tree2 leaf parent = {:?}", leaf2.parent.borrow().upgrade()); // now parent node exists, hence you'll get "Some(T)" as the answer
        println!("tree2 = {:?}\n", branch2); // now as there is a "weak" reference, there is no cycle and ":?" doesn't walk in loops and hence there is no stack overflow and no code crash
    } // "branch2" is going out of scope, hence its "Drop" trait is called and its "strong_count" is decremented
    // previously its "strong_count" is 1, now it becomes 0 and hence it gets cleaned up
    // while cleaning it up, it decrements the "strong_count" of "leaf2" as it was linked to "branch2" via "Rc:Clone" method
    println!("tree2 leaf(after branch goes out of scope), strong = {}, weak = {}", Rc::strong_count(&leaf2), Rc::weak_count(&leaf2));
    println!("tree2 leaf parent(after branch goes out of scope) = {:?}", leaf2.parent.borrow().upgrade()); // parent node when out of scope, hence you'll get "None" as the answer
}
