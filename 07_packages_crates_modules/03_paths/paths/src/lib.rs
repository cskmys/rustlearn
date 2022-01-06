mod front_of_house {
    pub mod hosting{ //  pub used to expose "hosting" to "front_of_house"
        pub fn add_to_waitlist(){ // pub used to expose "add_to_waitlist" to "hosting"

        }
    }
}

fn serve_order(){

}

mod back_of_house{
    pub struct Breakfast{ // from "back_of_house" you can access "Breakfast" (using "::" operator) as it is defined as "pub"
        pub toast: String, //  from "Breakfast" you can access "toast" (using "." operator) as it is defined as "pub"
        seasonal_fruit: String // from "Breakfast" you cannot access "seasonal_fruit" (using "." operator)
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast { // from "Breakfast" you can access "summer" (using "." operator) as it is marked as "pub"
            // if we didn't have this function code wouldn't compile coz this is the only way to instantiate the struct
            // the regular method cannot be used as "seasonal_fruit" is inaccessible to outside world
            // to instantiate a struct all fields need to be assigned
            Breakfast{
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches") // here you can access "seasonal_fruit" as "summer" is defined in context of "Breakfast"
                // hence "summer" has access to "Breakfast" and whatever comes under it meaning siblings of "summer" which in this case are the fields
            }
        }
    }

    pub enum Appetizer{ // "Appetizer" accessible to "back_of_house"
        Soup, // as enum is public, all variants are public even without using the keyword "pub" for each variant
        Salad
    }

    fn fix_incorrect_order(){
        cook_order();
        super::serve_order(); // code can access other code in the context that it is defined
        // here "fix_incorrect_order" can access "back_of_house" without "back_of_house" being defined with "pub" as "back_of_house" is the parent of "fix_incoorect_order"
        // it can also access siblings of its ancestors without them being defined with "pub"
        // hence, just using "super", "fix_incorrect_order" can access "serve_order"
    }
    fn cook_order(){

    }
}

pub fn eat_at_restaurant(){
    crate::front_of_house::hosting::add_to_waitlist(); // absolute path
    // "front_of_house" is a sibling to "eat_at_restaurant", so "front_of_house" can be called from "eat_at_restaurant" using absolute path.
    // as "hosting" is mentioned as "pub", "front_of_house" can access it
    // similarly "hosting" can access "add_to_waitlist"


    front_of_house::hosting::add_to_waitlist(); // relative path
    // here function "eat_at_restaurant" is a sibling of module "front_of_house", so it can access the module.
    // however, "eat_at_restaurant" can't access whatever is inside "front_of_house" unless "front_of_house" itself has access.
    // now as "hosting" is defined as "pub" it is choosing to expose itself to "front_of_house"
    // as "front_of_house" has access to "hosting" so does "eat_at_restaurant"
    // same reasoning applies to "eat_at_restaurant" accessing "add_to_waitlist"

    let mut meal = back_of_house::Breakfast::summer("Rye"); // can access method "summer" as it and "Breakfast" are defined as "pub"
    meal.toast = String::from("Wheat"); // can access field "toast" as it and "Breakfast" are defined as "pub"
    println!("I'd like {} toast please", meal.toast);
    /*
    meal.seasonal_fruit = String::from("Mango"); // cannot access field "seasonal_fruit" as it is not defined as "pub" though "Breakfast" is defined as "pub"
    */

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

/*
Module tree:
crate
 └── eat_at_restaurant
 └── serve_order
 └── front_of_house
 |   └── hosting
 |       └── add_to_waitlist
 └── back_of_house
     └── Breakfast
     |   └── toast
     |   └── seasonal_fruit
     |   └── summer
     └── fix_incorrect_order
     └── cook_order
*/
