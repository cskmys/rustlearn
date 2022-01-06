use std::io;
use std::collections::HashMap;

fn op_reg(reg: &HashMap<String, String>){
    for (name, dept) in reg{
        println!("{} => {}", name, dept);
    }
}

fn proc_reg_entry(reg: &mut HashMap<String, String>, entry: &String){
    let mut name = String::new();
    let mut dept = String::new();
    for (i, wd) in entry.split_whitespace().enumerate(){
        match i {
            1 => {
                name = String::from(wd);
            },
            3 => {
                dept = String::from(wd);
            },
            _ => {
                continue;
            }
        }
    }
    reg.insert(name, dept);
}

fn rd_usr_ip() -> String {
    let mut usr_ip = String::new();
    io::stdin()
        .read_line(&mut usr_ip)
        .expect("Failed to read line");
    String::from(usr_ip.trim())
}

fn main() {
    println!("Welcome to register..!");
    let mut reg: HashMap<String, String> = HashMap::new();
    reg.insert(String::from("amir"), "dir".to_string());
    loop {
        println!("Please input your command:");
        println!("0> manage register");
        println!("1> display register");
        println!("2> exit register");
        let usr_ip = rd_usr_ip();

        match &usr_ip[..] {
            "0" => {
                println!("You can do \"add <name> to <dept>\" to add employee");
                let reg_entry = rd_usr_ip();
                proc_reg_entry(&mut reg, &reg_entry);
            },
            "1" => {
                println!("Displaying register");
                op_reg(&reg);
            },
            "2" => {
                println!("Tata!!!");
                break;
            },
            _ => {
                println!("Please enter a valid command!");
                continue;
            }
        };

    }
}
