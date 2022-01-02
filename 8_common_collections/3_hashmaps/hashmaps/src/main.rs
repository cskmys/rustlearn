use std::collections::HashMap; // unlike "Vec" and "String" this is not automatically brought into scope
// hence you'll need to bring it into scope with "use" statement

fn main() {
    let team1 = String::from("Blue");
    let team2 = String::from("Yellow");
    let score1 = 10;
    let score2 = 50;
    let mut score_table = HashMap::new();
    score_table.insert(team1, score1); // if this line was not there, type inference would not work and we would have compilation error
    score_table.insert(team2, score2);
    /*
    println!("The teams are {} and {}", team1, team2); // throws compilation error
    // ownership of "team1" and "team2" is moved as "String" doesn't implement "copy" trait
    */
    println!("The scores are {} and {}", score1, score2); // ownership of "score1" and "score2" not lost as "i32" implements "copy" trait

    let team1 = String::from("Blue");
    let team2 = String::from("Yellow");
    let mut score_table = HashMap::new();
    score_table.insert(&team1, score1); // now "team1" does not lose ownership
    // however you'll need to make sure that "&team1" is valid at least during the whole lifetime of "score_table"
    // in other words, "score_table" is valid as long as "&team1" is valid
    score_table.insert(&team2, score2);
    println!("The teams are \"{}\" and \"{}\"", team1, team2); // "team1" and "team2" still has their ownership
    let teams = vec![team1, team2]; // "team1" and "team2" ownership lost
    /*
    score_table.clear(); // throws compilation error
    // "score_table" key is "&team1" and "&team2", hence "score_table" is valid as long as "&team1" and "&team2" are valid
    // during "let teams = vec![team1, team2];", "team1" and "team2" lost their ownership
    // hence "&team1" and "&team2" are no longer valid which makes "score_table" invalid as well
    */

    let scores = vec![score1, score2];
    let score_table: HashMap<_, _> = teams.into_iter().zip(scores.into_iter()).collect(); // creating immutable hashmap programmatically
    // creating mutable hashmap programmatically is not a big deal, but creating an immutable one programmatically is a lil bit more special
    // "into_iter" provides an iterator to a collection, in this case a vector
    // (NOTE: "into_iter" causes a move, hence "teams" and "scores" are no longer valid)
    // "zip" combines the iterators as a tuple
    // "collect" use the tuple of iterators to iterate through the whole collection and collect the vector elements as pairs
    // "collect" can return many data types, and compiler doesn't know which one to pick.
    // hence, "score_table" type is annotated as "HashMap<_, _>" but the key and value data-type is inferred

    let team1 = String::from("Red");
    let score = score_table.get(&team1);
    match score {
        Some(valid_score) => println!("The score is {}", valid_score),
        None => println!("invalid team name")
    };
    /*
    for (key, value) in score_table{ // score_table loses ownership here
        println!("{}:{}", key, value);
    }
    println!("score_table has {} elements", score_table.len()); // throws compilation error
    // "score_table" lost ownership as its key and value loses ownership when they are deconstructed as "(key, value)"
    */
    print!("score_table contains: ");
    for (key, value) in &score_table{ // score_table retains its ownership here
        print!("[\"{}\":{}] ", key, value);
    }
    print!("\n");
    println!("score_table has {} elements", score_table.len());
}
