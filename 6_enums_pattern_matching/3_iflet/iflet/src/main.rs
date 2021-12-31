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

fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The max configured is {}", max),
        _ => () // for the rest do nothing, here the rest is "None"
    } // too wordy

    // less verbose n concise way
    if let Some(max) = config_max { // if let <pattern> = <expression>{}
        println!("The max configured is {}", max);
    } // use of "if" automatically tells compiler do nothing for the rest("_ => ()")
    println!("{:#?}", config_max);


    let coin = Coin::Quarter(Country::Germany);
    let mut count = 0;
    match &coin {
        Coin::Quarter(country) => println!("The quarter was minted at {:#?}", country),
        _ => count += 1
    };

    /*
    if let Coin::Quarter(country) = coin { // causes compilation error
        println!("The quarter was minted at {:#?}", country);
    } else {
        count += 1;
    }
    dbg!(&coin); // throws compilation error
    // pattern matching in "if let" statement moves the ownership of "coin" and hence it is no longer accessible
    */
    if let Coin::Quarter(country) = &coin {
        println!("The quarter was minted at {:#?}", country);
    } else {
        count += 1;
    }
    dbg!(&coin);
}
