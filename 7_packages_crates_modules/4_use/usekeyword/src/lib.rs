mod front_of_house {
    pub mod hosting{
        pub fn add_to_waitlist(){

        }
    }
    pub mod nothing{
        pub fn doing_nothing(){

        }
    }
}

use crate::front_of_house::hosting; // brining an absolute path into scope with "use" statement
pub use self::front_of_house::nothing; // bringing a relative path into scope
// now any module that calls "nothing" will be referring to this "nothing" module as the path is define with "pub"
use crate::front_of_house::hosting::add_to_waitlist; // bringing a specific function into scope with absolute path
pub use self::front_of_house::nothing::doing_nothing; // bringing a specific function into scope with relative path
// now any module that calls "doing_nothing" will be calling this path as the statement is defined with "pub"

pub fn eat_at_restaurant(){
    hosting::add_to_waitlist(); // specifying parent makes it clear in locating the function
    nothing::doing_nothing();
    add_to_waitlist(); // as the function is brought into scope, no need to mention parent
    doing_nothing();
}

use std::fmt::Result;
/*
use std::io::Result; // throws compilation error
// when "Result" is used compiler won't know which one to pick
*/
use std::io::Result as IOResult; // now the alias "IOResult" can be used and there is no longer ambiguity in name resolution

