use std::fmt::Display;

struct Pair<T>{
    x: T,
    y: T
}

impl <T> Pair<T>{
    fn new(x: T, y: T) -> Self {
        Self{
            x,
            y
        }
    }
}

impl <T: Display + PartialOrd> Pair<T> {
    fn show_largest(&self){
        let mut largest = &self.x;
        if self.y > self.x {
            largest = &self.y;
        }
        println!("The largest value is {}", *largest);
    }
}

pub trait Summary {
    fn summarize(&self);
}

impl<T: Display> Summary for T { // conditional trait can be implemented on any type
    fn summarize(&self){
        print!("{}", self);
    }
}

impl<T:Summary> Summary for Pair<T>{
    fn summarize(&self){
        print!("x: "); self.x.summarize(); print!(", ");
        print!("y: "); self.y.summarize(); print!("\n");
    }
}

fn main() {
    let integer_pair = Pair {x: 5, y: 10};
    integer_pair.summarize();
    integer_pair.show_largest();

    let float_pair = Pair::new(5.0f32, 10.01);
    float_pair.summarize();
    float_pair.show_largest();

    let pair_of_integer_pair = Pair::new(Pair {x: 5, y: 10}, Pair {x: 15, y: 20});
    pair_of_integer_pair.summarize(); // will work coz "summarize" is defined for "T" and "Pair<T>" which effectively means "Pair<Pair<T>>" is also defined
    /*
    pair_of_integer_pair.show_largest(); // throws compilation error
    // type "Pair<T>" here in this case "Pair<int>" doesn't implement "PartialOrd" trait
    */
}
