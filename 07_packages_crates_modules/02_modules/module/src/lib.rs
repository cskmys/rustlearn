#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

mod front_of_house { // "mod" allows to define a module
    mod hosting{ // modules can be nested
        fn add_to_waitlist(){ // modules can hold functions, structs, enums, constants, traits

        }
        fn seat_at_table(){

        }
    }
    mod serving{ // can group all related functionality under a module
        fn take_order(){

        }
        fn serve_order(){

        }
        fn take_payment(){

        }
    }
}

/*
Module tree:

crate
 └── tests
 |   └── it_works
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment

it is very similar to filesystem.
here modules can be seen as folders and functions as files
*/