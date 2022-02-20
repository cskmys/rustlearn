use std::ops::Deref;

struct BoxS<T>(T); // creating our own smart pointer as a tuple struct with one element of type "T"

impl<T> BoxS<T>  {
    fn new(x: T) -> BoxS<T>{
        BoxS(x) // Unlike "Box<T>" which stores data on heap, "BoxS<T>" stores data on stack
    }
}

struct BoxSD<T>(T);

impl<T> BoxSD<T>  {
    fn new(x: T) -> BoxSD<T>{
        BoxSD(x)
    }
}

impl<T> Deref for BoxSD<T> {
    type Target = T; // the associated type for the "Deref" trait is "T"

    fn deref(&self) -> &Self::Target { // dereference operator "*" now performs its action on the data returned by this method
        &self.0 // in order for dereference operator "*" to behave normally we return the reference to the value we are trying to access
    }
    // hence when "*" is performed on "BoxSD<T>", it is being performed on "&self.0" i.e. "*(&self.0)" which gives "self.0"
}

fn main() {
    let x = 5;
    let bs = BoxS::new(x);

    assert_eq!(5, x);
    /*
    assert_eq!(5, bs); // throws compilation error
    // cannot compare "BoxS<T>" and "i32"
    */
    /*
    assert_eq!(5, *bs); // throws compilation error
    // cannot use dereference operator "*" on "BoxS<T>" as "Deref" trait is not implemented
    */

    let bsd = BoxSD::new(x);
    assert_eq!(5, *bsd); // the reference to "5" i.e. "&5" is being dereferenced i.e. "*(&5)", hence what is obtained is "5"
    // behind the scenes "*bsd" is interpreted as "*(bsd.deref())"

    let m = BoxSD::new(String::from("Rust(with deref coercion)"));
    hello(&m); // here "&m" of type "&BoxSD<String>" is translated as "BoxSD<String>::deref()" to get data of type "&String"
    // "&String" implements "Deref" trait too, and its "String::deref()" returns "&str"
    // hence "&String" is translated as "String::deref()" to get data of type "&str"

    let m = BoxSD::new(String::from("Rust(wo deref coercion)"));
    hello(&((*m)[..]));
    // here by "*m" gives "String" then "String[..]" to take the whole of it which will give you "str" and then do "&" on "str" to get "&str"
    // without deref coercion code is much harder to read
}

fn hello(name: &str){
    println!("Hello, {}!", name);
}