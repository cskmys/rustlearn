use std::io;

fn main() {
    println!("Enter 'n' to find nth fibonacci number");
    let n = loop {
        let mut n = String::new();
        io::stdin()
            .read_line(& mut n)
            .expect("Failed to read line");

        let n:u32 = match n.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Enter correct value");
                continue;
            }
        };
        if n != 0 {
            break n;
        } else {
            println!("Enter correct value");
        }
    };

    let mut prev_m1 = 0;
    let mut prev_0 = 1;
    let fib_n = if n == 1 {
        prev_m1
    } else {
        for _ in 1..n-2+1{
            let new_fib = prev_m1 + prev_0;
            prev_m1 = prev_0;
            prev_0 = new_fib;
        }
        prev_0
    };
    println!("the nth fibonacci number is {}", fib_n);
}
