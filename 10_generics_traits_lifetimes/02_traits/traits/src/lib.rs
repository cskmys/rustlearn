pub trait Summary { // "trait" keyword for a trait declaration, its public so it can be imported elsewhere
    fn summarize(&self) -> String; // method signature ends with ";" instead of "{" as implementation is defined in the type
    // any type that has a "Summary" trait must implement "summarize"
    fn test(&self) -> String {
        // default behavior of "test" trait
        format!("This test prints\n{}\nEnd of Test", self.summarize()) // a default method can call other non-default methods
    }
}

pub struct NewsArticle{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

impl Summary for NewsArticle{ // impl <trait_name> for <struct_name> { ... }
    fn summarize(&self) -> String {
        format!("{} by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet{ // trait "Summary" is in same scope, otherwise it would be: "impl Crate::Summary for Tweet{"
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn test(&self) -> String {
        String::from("this is a tweet test")
    }
}

pub fn notify(item: &impl Summary){ // "impl" means you pass a implementation of type "Summary"
    println!("Breaking news! {}", item.summarize());
}

fn returns_summarizable() -> impl Summary { // returning a type that implements trait "Summary"
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false
    }
}

/*
fn returns_summarizable_choice(choice: bool) -> impl Summary { // gets deduced as "fn returns_summarizable_choice<T: Summary>(choice: bool) -> T {"
    if choice{
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false
        }
    } else { // throws compilation error
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL.")
        }
    }
    // as "impl" gets translated as a generic
    // compiler interprets the first type that it comes across as the type of generic
    // hence here return type is interpreted as "Tweet"
}
*/

pub fn write_tweet(){
    let tweet = Tweet{
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false
    };
    println!("test new tweet: {}", tweet.test());
    println!("1 new tweet: {}", tweet.summarize());
    notify(&tweet);
}

pub fn write_article(){
    let article = NewsArticle{
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL.")
    };
    println!("test new article: {}", article.test());
    println!("1 new article: {}", article.summarize());
}