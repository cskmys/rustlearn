use std::{thread, time::Duration};

struct Cacher<T> // closure is defined as a trait bound on the generic parameter of the structure
where T: Fn(u32) -> u32 { // here closure type is required
    calculation: T, // both fields are private as we don't want them to be modified from outside
    value: Option<u32>
}

impl<T> Cacher<T>
where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None
        }
    }
    fn value(&mut self, arg: u32) -> u32{
        // before executing the closure, "value" will be "None", after executing it "Some(u32)" is stored
        match self.value {
            Some(v) => v, // for subsequent times you can just return the value without executing the closure
            None => { // hence, for the first time you can execute closure, cache the value and return it
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    let example_closure = |x| x;

    println!("from closure: {}",example_closure(5)); // the first usage has an "i32" data passed, hence compiler infers the closure as "|x: i32| -> i32 { x }"
    /*
    let s = example_closure(String::from("hello")); // throws compilation error
    // at the first usage "i32" type was inferred and "String" is being passed in place of "i32"
    */


    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    generate_workout(25, 3);
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32){
    let mut expensive_result = Cacher::new(|nb|{ // each time the function is called new "Cacher" is built
        simulated_expensive_calculation(nb)
    }); //  each function call "expensive_result" is of different type from the previous function call as each closure is its unique type
    // however, here that doesn't matter much as the "expensive_result" is dropped at the end of the scope

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity)); // first time the closure is executed and result is cached
        println!("Next, do {} situps!", expensive_result.value(intensity)); // now, the expensive computation is not executed twice, cached value is just returned
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!"); // now we are no longer calling the expensive computation
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}