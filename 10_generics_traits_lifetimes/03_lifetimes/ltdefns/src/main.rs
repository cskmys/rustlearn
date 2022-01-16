use std::fmt::Display;

struct ImportantExcerpt<'a> { // indicating lifetime just like generic type is declared
    part: &'a str // indicates the lifetime of the data that the "part" is referencing to should be at least the lifetime of the struct "ImportantExcerpt"
}

impl<'a> ImportantExcerpt<'a> { // specifying lifetime using "<'a>" is required after "impl" and "ImportantExcerpt"
    fn level(&self) -> i32 { // first elision rule hence specifying lifetime for "self" is not required
        3
    }

    fn announce_n_return_part(&self, announcement: &str) -> &str { // third elision rule hence specifying lifetime for "self" and "announcement" is not required
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann:T) -> &'a str
    where T:Display { // both lifetimes and trait types are generic parameters hence they are written together as "<'a, T>"
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Couldn't find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence
    };
    println!("novel: \"{}\"", novel);
    println!("first sentence: \"{}\"", first_sentence);
    let ret = longest_with_an_announcement(&novel, first_sentence, i.part);
    println!("longest sentence: \"{}\"", ret);
    println!("important excerpt(a): \"{}\"", i.part);
    /*
    let first_sentence;
    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        first_sentence = novel.split('.').next().expect("Couldn't find a '.'"); // throws compilation error
    } // "novel" goes out of scope, hence "first_sentence" is a dangling reference
    let i = ImportantExcerpt{
        part: first_sentence // "first_sentence" is a invalid
        // hence this violates the lifetime rule defined in the struct i.e. "first_sentence" should be valid at least the time during which "i" is valid
    };
    */
    {
        let i = ImportantExcerpt {
            part: first_sentence
        };
        println!("important excerpt(b): \"{}\"", i.part);
    } // "i" becomes invalid but "first_sentence" is still valid, hence there was no violation of struct lifetime

    let s: &'static str = "lifetime during the entirety of the program";
    let i = ImportantExcerpt {
        part: s
    };
    println!("important excerpt(c): \"{}\"", i.announce_n_return_part("blah"));
    println!("level: {}", i.level());
}
