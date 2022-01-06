mod front_of_house; // using ";" instead of "{" means that content of module needs to loaded from another file
// here as this is crate root module, module "front_of_house"is searched for in file "front_of_house.rs" under "src" folder.

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant(){
    hosting::add_to_waitlist();
}