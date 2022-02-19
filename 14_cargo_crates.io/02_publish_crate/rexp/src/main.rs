use rexp::kinds::PrimaryColor; // code that uses library will have to go through whole hierarchy to access an item
// unlike the developer, for the library user "kinds" and "utils" is not of too much relevance

use rexp::{SecondaryColor, mix}; // as "SecondaryColor" and "mix" were re-exported, we can directly import it here as if they were at the top of the hierarchy
// this prevents library user from having to go through hierarchy to find a function

fn main(){
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    let orange = mix(red, yellow);
    assert_eq!(orange, SecondaryColor::Orange);
}