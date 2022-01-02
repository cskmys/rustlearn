fn main() {
    let txt = "Salut";
    let sf = txt.to_string();
    let ss = String::from("नमस्ते"); // "String" type is UTF-8 encoded
    println!("\"Hi\": French(created from \"to_string\"): \"{}\", Sanskrit(created from \"String::from\"): \"{}\"", sf, ss);

    let mut s = String::new(); // creating a new empty string
    println!("s(empty string): \"{}\"", s);
    s.push_str(txt); // "push_str" takes in "&str" hence, it does not take the ownership of "txt", and "txt" can be used later
    println!("s(after pushing string): \"{}\"", s);
    s.push('!');
    println!("s(after pushing character): \"{}\"", s);
    let txt = String::from("Ciao!");
    /*
    s = txt + s; // throws compilation error here
    // operator "+" is implemented as "fn add(self, &str) -> String {"
    // hence, whatever is on rhs of "+" is passed as "&str" and "s" is "String"
    */
    s = txt + &s; // "&s" which is "&String" gets interpreted as "&s[..]" which is "&str" due to dereference coercion
    /*
    println!("s(after concatenating it to string \"{}\"): \"{}\"", txt, s);
    // operator "+" is implemented as "fn add(self, &str) -> String {"
    // hence, whatever is on lhs of "+" is passed as "self" and causing it to lose its ownership
    // "txt" is no longer valid
    */
    println!("s(after pre-concatenating another string): \"{}\"", s);
    let txt = String::from("Hola!");
    s = format!("{}{}", txt, s); // unlike "+", "format!" macro uses all references
    // Hence, it doesn't cause loss of any ownership and moreover, it is more flexible in wysiwyg form
    // Hence, use "format!" macro in place of "+"
    println!("s(after pre-concatenating \"{}\"): \"{}\"", txt, s); // now "txt" still has its ownership and can be used as well
}
