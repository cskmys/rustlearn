fn main() {
    let lyric = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "five gold rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming"
    ];
    for i in 0..lyric.len(){
        println!("On the first day of Christmas my true love sent to me");
        for j in (0..i+1).rev(){
            println!("{}", lyric[j]);
        }
        println!();
    }
}