use std::collections::HashMap;

fn main() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let scores = vec![10, 50];
    let mut score_table: HashMap<_, _> = teams.into_iter().zip(scores.into_iter()).collect();

    println!("score_table(initial): {:?}", score_table);

    score_table.insert(String::from("Blue"), 15);
    println!("score_table(after overwriting score): {:?}", score_table);

    score_table.entry(String::from("Blue")).or_insert(20); // if "Blue" key already exists do nothing
    // "entry" return an enum "Entry" which says if something exists or not
    // "or_insert" returns a mutable reference for the mapped value of the key if the entry exists
    // otherwise it maps and inserts the new value for the key and returns a mutable reference to the new value
    // using this construct is way cleaner than writing the logic ourselves wherein we would have needed to pay attention to borrow or ownership etc
    score_table.entry(String::from("Red")).or_insert(20); // if "Red" key does not exist insert the entry
    println!("score_table(after conditional insert): {:?}", score_table); // throws compilation error

    /*
    let blue_score = score_table.entry(String::from("Blue")).or_insert(20); // returns "&15" as "Blue" key already exists
    // first mutable borrow on a value happened here, but compiler consider the borrow to have happened on the whole hashmap
    let red_score = score_table.entry(String::from("Red")).or_insert(20); // expected to return "&20" as "Read" key already exists but compilation error
    // another mutable borrow happened here but on a different value, still compiler considers this as a 2nd mutable borrow on the whole hashmap
    // hence, "blue_score" gets invalidated
    println!("score_table(again): {:?}", score_table); // here the whole "score_table" is borrowed as immutable
    // using an immutable borrow invalidates any mutable borrow occurring above it, this invalidates "red_score" too along with already invalidated "blue_score"
    println!("blue_score: {}, red_score: {}", blue_score, red_score); // throws compilation error as we are using invalid references
    */

    let blue_score = score_table.entry(String::from("Blue")).or_insert(20); // returns "&15" as "Blue" key already exists
    println!("blue_score: {}", blue_score);
    let red_score = score_table.entry(String::from("Red")).or_insert(20); // returns "&20" as "Read" key already exists and "blue_score" invalidated
    println!("red_score: {}", red_score);
    println!("score_table(again): {:?}", score_table); // "red_score" invalidated along with already invalid "blue_score"

    // this return feature can be used to build some interesting use-cases
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count += 1; // need to deference as whatever we have is a reference
    }
    println!("the string is \"{}\"", text);
    println!("its word frequency table is: {:#?}", map);
}
