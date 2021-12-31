fn main() {
    let mut s = String::from("Heyyy world");
    let word = first_word(&s);
    println!("String is \"{}\" and it's first word ends at {}", s, word);

    s.clear();
    println!("String is \"{}\" and it's first word ends at {}", s, word); // wrong output
    // "first_word" needs to be executed again as "word" is not linked to state of "s", "word" is not updated when "s" changes
    // if we need "word" to be in sync with state of "s", it can be difficult and error prone with the above mentioned method

    let mut s = String::from("Hello world");
    // slice declaration = &<collection_variable>[desired_first_index..desired_last_index+1]
    let word1 = &s[0..5]; // can also be written as &s[..5] first index can be omitted to indicate the start of the collection
    // "&s[0..5]" is a reference to elements:
    // 1. at index 0: 'H'
    // 2. between element at index 0 and 5: characters between 'H' and ' ': 'e', 'l', 'l', 'o'
    // Hence, "word1" is a reference to elements: 'H', 'e', 'l', 'l', 'o'
    let word2 = &s[6..11]; // can also be written as &s[6..] last index can be omitted to indicate the end of the collection
    let whole_string = &s[..];
    println!("Slice 1: \"{}\", Slice 2: \"{}\", Whole string: \"{}\"", word1, word2, whole_string);
    // NOTE: Here all strings are ascii encoded which means one byte to one character, hence we can create slice using the position of the character
    // if string was UTF-8 encoded then each character would be 4 bytes, then slice indices should be multiples of 4, otherwise an error will thrown during runtime

    let word1 = first_word_slice(&s);
    println!("First word: \"{}\"", word1);
    s.clear();
    /*
    println!("First word: \"{}\"", word1); // causes a compilation error

    "clear" function declares &mut s, which is a mutable reference of s.
    when a mutable reference is declared all previous immutable references are invalidated.
    hence, word1 which was an immutable reference is marked invalid and is no longer usable.
    Therefore, by mistake if you use "word1" again after the state of "s" is changed without recalculating "word1", you get an error that too at compilation time!
    */
    let word1 = first_word_slice(&s); // compiler error forces you to recompute "word1", therefore no room for state errors
    println!("First word: \"{}\"", word1);


    let s_string = String::from("Oi world");
    let word1_string = first_word_slice_gen_ver(&s_string); // you can also pass as a slice &s_string[..]
    println!("s(String type): \"{}\", First word: \"{}\"", s_string, word1_string);
    let s_str = "Oi world";
    let word1_str = first_word_slice_gen_ver(&s_str); // you can also pass as a slice &s_str[..]
    println!("s(str type): \"{}\", First word: \"{}\"", s_str, word1_str);

    let hello = ['h', 'e', 'l', 'l', 'o'];
    let slice = &['h', 'e', 'l', 'l']; // slices can be created off the bat this way too
    println!("&['h', 'e', 'l', 'l', 'o'][..4] == &['h', 'e', 'l', 'l'] is {}", &hello[..4]==slice);
}

fn first_word(s: &String) -> usize{
    let bytes = s.as_bytes(); // converting string into bytes to iterate element by element to check for space

    for (i, &item) in bytes.iter().enumerate(){ // iter returns each element in collection everytime it is called
        // enumerate wraps result of iter into a tuple which is destructured by pattern (i, &item)
        if item == b' ' { // if current byte is space checked using byte literal syntax
            // NOTE: Here all strings are ascii encoded which means one byte to one character
            // if string was UTF-8 encoded it won't work
            return i-1;
        }
    }

    s.len() - 1
}

fn first_word_slice(s: &String) -> &str{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_slice_gen_ver(s: &str) -> &str{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}