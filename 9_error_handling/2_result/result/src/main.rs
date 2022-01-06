use std::fs::{self, File};
use std::io::{self, Read, ErrorKind, Write};
use std::error::Error;

fn advanced_way(file_name: &str) -> File {
    let f = File::open(file_name).unwrap_or_else(|err|{
        if err.kind() == ErrorKind::NotFound {
            File::create(file_name).unwrap_or_else(|err|{
                panic!("Problem creating file {:?}", err);
            })
        } else {
            panic!("Problem opening file {:?}", err);
        }
    });
    return f;
}

fn beginner_way(file_name: &str) -> File {
    let f = File::open(file_name); // The return type of opening a file is a "Result<T, E>"
    // Here "T" is file handle "std::fs::File" and "E" is error value "std:io:Error"
    let f = match f {
        Ok(file) => file, // we can directly use "Ok" instead of "Result::Ok" as "Result" is automatically brought to scope just like "Option" enum
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create(file_name) {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file {:?}", e),
            },
            other_error => panic!("Problem opening the file {:?}", other_error),
        }
    };
    return f;
}

fn read_username_from_file(file_name: &str) -> Result<String, io::Error>{ // return type is "Result<T, E>" where "T" is "String" and "E" is "io::Error"
    let f = File::open(file_name);
    let mut f = match f{
        Ok(file) => file,
        Err(e) => return Err(e), // return "Err(E)" instead of doing "panic!"
    };

    let mut s = String::new();
    match f.read_to_string(&mut s){ // "read_to_string" also returns "Result<T,E>" however, the output string is loaded to argument passed as a reference
        Ok(_) => Ok(s), // wrapping the output string in "Ok"
        Err(e) => Err(e),
    }
}

fn read_username_from_file_short(file_name: &str) -> Result<String, io::Error>{
    let mut f = File::open(file_name)?;
    // "?" works the same way as if a match expression was written:
    /*
        let mut f = match File::open(file_name){
            Ok(val) => val,
            Err(e) => return Err(e),
        }
    */
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    // "?" works the same way as if a match expression was written:
    /*
        match f.read_to_string(&mut s){
            Ok(val) => val, // "val" is not collected as the desired output is received in "&mut s"
            Err(e) => return Err(e),
        }
    */
    // error values that have the "?" operator called on them go through the "from" function, defined in the "From" trait in the standard library, which is used to convert errors from one type into another.
    // When the "?" operator calls the "from" function, the error type received is converted into the error type defined in the return type of the current function.
    // This is useful when a function returns one error type to represent all the ways a function might fail, even if parts might fail for many different reasons.
    // As long as each error type implements the "from" function to define how to convert itself to the returned error type, the "?" operator takes care of the conversion automatically.
    Ok(s)
}

fn read_username_from_file_advanced(file_name: &str) -> Result<String, io::Error>{
    let mut s = String::new();
    File::open(file_name)?.read_to_string(&mut s)?;
    // equivalent to writing
    /*
        match File::open(file_name){
            Ok(f) => {
                match f.read_to_string(&mut s){
                    Ok(val) => val,
                    Err(err) => return Err(err)
                }
            },
            Err(e) => return Err(e),
        }
    */
    Ok(s)
}

fn read_username_from_stdlib(file_name: &str) -> Result<String, io::Error>{
    fs::read_to_string(file_name) // as it is one of the common operations performed, the std library has an API for it
}

fn main() -> Result<(), Box<dyn Error>>{ // the return type of "main" is "()"
    let file_name = "hello.txt";
    let mut f = advanced_way(file_name); // beginner_way
    // let s = read_username_from_file(file_name).expect("Read failed");
    // if let Err(err) = f.write("1".as_bytes()){
    //     panic!("Write failed {:?}", err);
    // }
    // f.write("1".as_bytes()).unwrap();
    // f.write("1".as_bytes()).expect("File write failed"); // causes panic as well but adds "File write failed" msg

    /*

    f.write("1".as_bytes())?; // throws compilation error when "fn main(){" is the "main" function signature

    // "?" operator can be used only in a function that returns "Result" or "Option" or another type that implements "std::ops::Try"
    // when a function doesn't return any of the above type,
    // the simplest solution is to change the return type of the function to "Result<T, E>" if there are no restrictions on the function.
    // another technique is to use "match" or one of the methods of "Result<T, E>" to handle it like "expect", "unwrap" etc
    */

    f.write("1".as_bytes())?; // in this case program doesn't crash per se but returns the error to OS
    Ok(())
}
