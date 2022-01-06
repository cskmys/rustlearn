#[derive(Debug)] // needs to included for "IpAddrKind" if we want to use it for other complex data type which wants to use it
enum IpAddrKind { // "IpAddrKind" is a custom data type
    // "IpAddrKind" can have one of the values among the ones defined below
    v4,
    v6
}

#[derive(Debug)]
struct IpAddr{
    kind: IpAddrKind,
    address: String
}

#[derive(Debug)]
enum IpAddrE{
    v4(String), // can be used in place of "<var> = IpAddr{kind:IpAddrKind::v4, address:<addr_string>}"
    v6(String)
} // this is equivalent of having an enum where the user needs to pick one out of two structs "v4" and "v6" where "v4" has field "String" and "v6" has field "String"

#[derive(Debug)]
enum IpAddrDiff {
    v4(u8, u8, u8, u8),
    v6(String),
}

enum Message{
    Quit, // enum value for data of type unit struct "struct <struct_name>;"
    Move { // enum value for data of type regular struct "struct <struct_name>{x: i32, y: i32}"
        x: i32,
        y: i32
    },
    Write(String), // enum value for data of type tuple struct "struct <struct_name>(String);"
    ChangeColor(i32, i32, i32), // enum value for data of type tuple struct "struct <struct_name>(i32, i32, i32)"
}

impl Message {
    fn call(&self){
        // how to know which type of self it is "Quit", "Move", "Write" or "ChangeColor"?
        println!("Called!");
    }
}

fn main() {
    let four = IpAddrKind::v4;
    let six = IpAddrKind::v6;

    route(IpAddrKind::v4); // called using a constant value
    route(six); // a variable passed by value

    let home = IpAddr{
        kind: four,
        address: String::from("127.0.0.1")
    };
    dbg!(home);
    /*
    let loopback = IpAddr{
        kind: four, // throws compilation error
        address: String::from("::1")
    };
    // "four" is an enum a complex data type, hence it has "move" trait not "copy" trait.
    // therefore, "four" got moved when "let home = IpAddr{kind: four," was done and is no longer accessible
    */

    /*
    let loopback = IpAddr{
        kind: six, // throws compilation error
        address: String::from("::1")
    };
    // "six" is an enum a complex data type, hence it has "move" trait not "copy" trait.
    // therefore, "six" got moved during the function call "route(six);" and is no longer accessible
    */
    let loopback = IpAddr{
        kind: IpAddrKind::v6,
        address: String::from("::1")
    };
    dbg!(loopback);

    let home = IpAddrE::v4(String::from("127.0.0.1")); // though it can be used instead of "let home = IpAddr{kind: four,address: String::from("127.0.0.1")};", it is not exact equivalent of it
    let loopback = IpAddrE::v6(String::from("::1"));
    dbg!(home);
    dbg!(loopback);

    let home = IpAddrDiff::v4(127, 0, 0, 1);
    let loopback = IpAddrDiff::v6(String::from("::1"));
    dbg!(home);
    dbg!(loopback);

    let m = Message::Write(String::from("hello"));
    m.call();
}

fn route(ip_kind: IpAddrKind){ // pass by value of an enum which is a complex datatype, hence ownership will be acquired by this function
    dbg!(ip_kind); // ownership of ip_kind moved to "dbg!" macro and lost by the function
}