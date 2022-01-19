use std::{thread, time::Duration};

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32){
    let expensive_closure = |val| {
        simulated_expensive_calculation(val) // moving the function call inside a closure
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity)); // wherever closure is used, only there expensive computation is done
        println!("Next, do {} situps!", expensive_closure(intensity)); // but now, we reintroduced problem of calling expensive computation twice
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
            // now we are no longer calling the expensive computation
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}