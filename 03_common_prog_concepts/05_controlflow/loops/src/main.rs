fn main() {
    let mut count = 0;
    'counting_up: loop { // outer loop
        println!("count = {}", count);
        let mut remaining = 10;

        loop { // inner loop
            println!("remaining = {}", remaining);
            if remaining == 9{
                break; // breaks at the inner loop
            }
            if count == 2 {
                break 'counting_up; // breaks from the outer loop
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10{
            break counter * 2; // break returns a value
        }
    };
    println!("The result is {}", result);

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in &a { // "&" coz "a" is an array and not an iterator
        println!("the value is: {}", element);
    } // safe and concise than using a while to iterate

    for number in (1..4).rev(){ // (1..4) is a range from [1,3] and rev provides a reverse iterator to that range
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
