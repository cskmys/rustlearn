const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn word_en_to_pig(wd: &str) -> String { // using string slice type "&str" instead of "&String" makes it more generic by allowing usage with both "String" and "str"
    if let Some(first) = wd.chars().into_iter().next(){
        if VOWELS.contains(&first) {
            format!("{}-{}", wd, "hay")
        } else {
            format!("{}-{}{}", &wd[first.len_utf8()..], first,"ay")
        }
    } else {
        String::new()
    }
}

fn str_en_to_pig(s_en: &str) -> String{
    let mut s_en_itr = s_en.split_whitespace().into_iter();
    if let Some(first_wd) = s_en_itr.next(){
        let mut s_pg = word_en_to_pig(first_wd);
        for wd_en in s_en_itr {
            let wd_pg = word_en_to_pig(wd_en);
            s_pg = format!("{} {}", s_pg, wd_pg)
        }
        s_pg
    } else {
        String::new()
    }
}

fn main() {
    let wd_en = "hello";
    println!("EN:\"{}\", PG: \"{}\"", wd_en, word_en_to_pig(wd_en));
    let wd_en = "apple";
    println!("EN:\"{}\", PG: \"{}\"", wd_en, word_en_to_pig(wd_en));

    let s_en = "hello apple wonderful world";
    println!("EN:\"{}\", PG: \"{}\"", s_en, str_en_to_pig(s_en));
}