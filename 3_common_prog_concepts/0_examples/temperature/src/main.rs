use std::io;

fn main() {
    println!("Choose a conversion option");
    println!("0: Fahrenheit to celsius");
    println!("1: Celsius to fahrenheit");
    let opt = loop {
        let mut opt = String::new();
        io::stdin()
            .read_line(&mut opt)
            .expect("Failed to read line");

        let opt:u32 = match opt.trim().parse() {
            Ok(num) => {
                num
            },
            Err(_) => {
                println!("Type a proper choice");
                continue;
            }
        };
        if opt >= 2 {
            println!("Type a proper choice");
            continue;
        }
        break opt;
    };

    println!("Enter the temperature");
    let temp = loop {
        let mut temp = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        let temp:f64 = match temp.trim().parse() {
            Ok(num) => {
                num
            },
            Err(_) => {
                println!("Type a proper choice");
                continue;
            }
        };
        break temp;
    };

    let conv_temp = if opt == 0 {
        fahrenheit_to_celsius(temp)
    } else {
        celsius_to_fahrenheit(temp)
    };
    println!("Converted temperature is {}", conv_temp);
}

fn fahrenheit_to_celsius(fah_temp:f64) -> f64 {
    (fah_temp - 32.0) * 5.0/9.0
}

fn celsius_to_fahrenheit(cel_temp:f64) -> f64 {
    (cel_temp * 9.0/5.0) + 32.0
}