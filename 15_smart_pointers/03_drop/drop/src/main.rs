struct CustomSmartPointer {
    data: String
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self){
        println!("Dropping CustomSmartPointer with data '{}'!", self.data);
    }
}

fn func(){
    let c = CustomSmartPointer{
        data: String::from("some data"),
    };
    println!("func(): CustomSmartPointer created!");
    /*
    c.drop(); // throws compilation error
    // "drop" function of "Drop" trait cannot be called manually
    */
    drop(c); // "std::mem::drop" is used to clear the memory before end of scope
    // this "drop" is from "std::mem" which is part of prelude
    // hence we didn't need to import it before using it
    println!("CustomSmartPointer dropped before the end of the func");
}

fn main() {
    func();
    let c = CustomSmartPointer{
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer{
        data: String::from("other stuff"),
    };
    println!("main(): CustomSmartPointers created!");
} // for "c" and "d", "Drop" trait's "drop" methods are called
// "d" is dropped first and then followed by "c"