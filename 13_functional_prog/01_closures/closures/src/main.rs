use std::{thread, time::Duration, collections::HashMap};

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
    fn value(&mut self, arg: u32) -> u32 {
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

#[test] // you can directly do ad-hoc unit testing here in "main.rs" without splitting code into a library  in new file "src/lib.rs" and creating a "tests" module
fn call_with_different_values(){
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1); // first time executed and return is cached
    assert_eq!(v1, 1);

    let v2 = c.value(2); // 1 was cached, so 1 is returned, though a new parameter value is passed
    assert_eq!(v2, 2); // fails coz 1 was returned
}

struct CacherAdv<T>
    where T: Fn(u32) -> u32 {
calculation: T,
value_map: HashMap<u32, Option<u32>> // using hashmap to solve the problem with the simple "Cacher"
}

impl<T> CacherAdv<T>
    where T: Fn(u32) -> u32 {
    fn new(calculation: T) -> CacherAdv<T> {
        CacherAdv {
            calculation,
            value_map: HashMap::new()
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value_map.get(&arg) {
            Some(&v) => {
                match v {
                    Some(n) => n,
                    None => {
                        0
                    }
                }
            }, // for subsequent times you can just return the value without executing the closure
            None => { // hence, for the first time you can execute closure, cache the value and return it
                let v = (self.calculation)(arg);
                self.value_map.insert(arg, Some(v));
                v
            }
        }
    }
}

#[test]
fn call_with_different_values_adv(){
    let mut c = CacherAdv::new(|a| a);

    let v1 = c.value(1);
    assert_eq!(v1, 1);

    let v2 = c.value(2); // now due to use of hashmap, any new value will result in re-computation
    assert_eq!(v2, 2);
}

fn main() {
    let y = 5;
    let plus_y = |x| x + y;

    println!("from closure(primitive): {}", plus_y(5)); // the first usage has an "i32" data passed, hence compiler infers the closure as "|x: i32| -> i32 { x + y }"
    println!("y(primitive): {}", y);
    /*
    let s = example_closure(String::from("hello")); // throws compilation error
    // at the first usage "i32" type was inferred and "String" is being passed in place of "i32"
    */
    /*
    fn plus_y_fn(x: i32) -> i32 {
        x + y // throws compilation error
    }
    // only closures can access the environment in which they are defined
    */
    let plus_y_move = move |x| x + y;
    println!("from move closure(primitive): {:?}", plus_y_move(5)); // "move" tells the closure to take ownership but for primitive data it just does a copy
    println!("y(primitive): {:?}", y); // "y" got copied in the closure rather than getting moved, hence it is still accessible

    let y = vec![5, 2, 3];
    let push_y = |x| { let mut z = y.clone(); z.push(x); z};
    println!("from closure(complex): {:?}", push_y(5));
    println!("y(complex): {:?}", y);

    let push_y_move = move |x| { let mut z = y.clone(); z.push(x); z}; // "move" tells the closure to take ownership of the complex data
    println!("from move closure(complex): {:?}", push_y_move(5)); // "y" is moved
    /*
    println!("y(complex): {:?}", y); // throws compilation error
    // closure took over the ownership of "y", hence it is not accessible from outside of closure anymore
    */

    println!("");

    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
    generate_workout(25, 3);
    generate_workout(30, simulated_random_number + 1);
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32){
    let mut expensive_result = CacherAdv::new(|nb|{ // Advanced cacher
        simulated_expensive_calculation(nb)
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!"); // now we are no longer calling the expensive computation
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}