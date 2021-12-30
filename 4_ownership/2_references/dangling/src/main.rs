fn main() {
    let dang_ref = dangle_ex();
}

/*
fn dangle_ex() -> &String { // throws compilation error
    let s = String::from("Hello, world!"); // memory with string bound to s
    &s // reference to memory is returned
} // memory bound to s is getting deallocated while reference to it is being returned
// hence dangling ref gets created if compiler doesn't throw error.
// luckily rust compiler throws error to this
*/

fn dangle_ex() -> String {
    let s = String::from("Hello, world!"); // memory with string bound to s
    s // variable s is returned not any reference
} // s ownership is being moved from out of function into the function who called this function
// hence there is no de-allocation of memory which was bound to variable s