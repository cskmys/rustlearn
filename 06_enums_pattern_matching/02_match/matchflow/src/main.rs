#[derive(Debug)]
enum Country{
    France,
    Germany,
    Spain,
    Italy
}

#[derive(Debug)]
enum Coin{
    Penny,
    Nickel,
    Dime,
    Quarter(Country) // Quarters in EU have logo of country where they were minted
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5, // comparison happens in order and the first match gets executed
        Coin::Dime => 10,
        Coin::Quarter(country) => {
            println!("The quarter was minted at {:#?}", country);
            25
        },
    }
}

fn add_fancy_hat(){
    println!("wear the hat");
}
fn rm_fancy_hat(){
    println!("remove the hat");
}
fn mv_player(nb_places: u8){
    println!("move {} places", nb_places)
}
fn re_roll(){
    println!("roll again");
}

fn manage_dice_roll(roll: u8){
    match roll {
        3 => add_fancy_hat(),
        5 => rm_fancy_hat(),
        other=> mv_player(other) // "other" is a catch all pattern which is always put at the last
        // coz anything that doesn't match is caught by this which means any pattern after this will be useless
    };
    match roll {
        3 => (), // unit tuple here means do nothing
        5 => (),
        _ => re_roll() // catch all using patterns, value of roll is not bound to anything
    };
}

fn plus_one(x: Option<i32>) -> Option<i32>{
    /*
    match x { // throws compilation error
        Some(i) => Some(i + 1)
    }
    // not all the possible values of x have been addressed in the arms
    // hence you get compilation error
    */
    match x {
        None => None,
        Some(i) => Some(i + 1) // x binds to i
    }
}

fn main() {
    let coin = Coin::Penny;
    println!("The coin's value in cent(s) is {}", value_in_cents(&coin));
    let coin = Coin::Quarter(Country::France);
    println!("The coin's value in cent(s) is {}", value_in_cents(&coin));

    let five = Some(5); // "Option<T>" takes datatype based on "T"
    // Here "T" is "i32" which is a primitive data type, hence "Option<T>" acts as primitive type and gets copied rather than moved
    let six = plus_one(five); // no need of reference, can directly pass by value as "Option<i32>" gets copied like "i32"
    let none = plus_one(None);

    manage_dice_roll(3);
    manage_dice_roll(4);

    let coin = Coin::Quarter(Country::Germany);
    let mut count = 0;
    /*
    match coin { // causes compilation error
        Coin::Quarter(country) => println!("The quarter was minted at {:#?}", country),
        _ => count += 1
    }; // during pattern matching the ownership of "coin" gets moved while binding it to pattern
    dbg!(&coin); // throws compilation error here
    */
    match &coin {
        Coin::Quarter(country) => println!("The quarter was minted at {:#?}", country),
        _ => count += 1
    };
    dbg!(&coin);

}
