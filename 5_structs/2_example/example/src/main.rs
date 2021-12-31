struct Rectangle{
    width: u32,
    height: u32
}

#[derive(Debug)]
struct RectangleDbg {
    width: u32,
    height: u32
}


fn main() {
    let width = 30;
    let height = 50;
    println!("The area of rect(using independent variables) is {} square pixels", area(width, height));

    let dim = (width, height);
    println!("The area of rect(using tuple) is {} square pixels", area_tuple(dim));

    let rect = Rectangle{
        width,
        height
    };
    println!("The area of rect(using struct) is {} square pixels", area_struct(rect));
    /*
    println!("The width of rect is {} pixels", rect.width); // throws compilation error
    // cause "rect" was moved while "area_struct" function was called
    */
    let rect = Rectangle{
        width,
        height
    };
    println!("The area of rect(using struct and passing it by ref) is {} square pixels", area_struct_ref(&rect));
    println!("The width of rect is {} pixels", rect.width);
    /*
    println!("The rect is {} pixels", rect); // throws compilation error
    // cause "println!" does not know how to(in which order, and formatting like what separator etc, etc) access the fields of "rect" to print its values
    */
    // the "{}" in "println!("{}", <var>);" asks "println!" to use formatting known as "Display" which outputs the variable for end user to see on the screen
    // the primitive data types have "Display" trait implemented, but complex data types do not have this implemented
    // However, rust has another inbuilt option called pretty print which prints the structs and other complex data types in its own particular format.
    // to use it include ":?" or ":#?" inside "{}" while using it in "println!" macro: "println!("The rect is {:?} pixels", rect);"
    // this would still throw compilation error coz ":?" or ":#?" can be used only if "Debug" trait is implemented for "rect"'s type "Rectangle"
    // rust provides a default functionality for "Debug" trait that allows to see the fields of struct.
    // to avail this functionality, outer attribute "#[derive(Debug)]" must be added before declaration of struct
    let rect = RectangleDbg{
        width,
        height
    };
    println!("The rect dimensions is {:?}", rect);
    println!("The rect dimensions(now with whitespace formatting) is {:#?}", rect);
    // "println!" prints on "stdout" but normally debug outputs are printed on "stderr"
    // to print on "stderr" you can use "dbg!" macro which prints the default debug output that would be printed by using "{:#?}" in "println!" and the file name and line number
    /*
    dbg!(rect);
    println!("The rect dimensions(now with whitespace formatting) is {:#?}", rect); // "dbg!(rect);" causes compilation error here
    // "dbg!" macro takes ownership of whatever you pass to it, hence pass a reference not the variable
    */
    dbg!(&rect);
    println!("The rect dimensions(with whitespace formatting again after using \"dbg!\" macro) is {:#?}", rect);
    let scale = 2;
    let scaled_rect = RectangleDbg{
        width: dbg!(width * scale), // "dbg!" macro returns whatever you passed to it
        height: height * scale
    };
    dbg!(&scaled_rect);
}

fn area(width: u32, height: u32) -> u32{
    width * height // independent variables does not imply any relationship between them
}

fn area_tuple(dim: (u32, u32)) -> u32{
    dim.0 * dim.1 // now there is some organization but indexing doesn't indicate which is what
}

fn area_struct(rect: Rectangle) -> u32{
    rect.width * rect.height // now everything is very clear but function caller loses ownership of rect
}

fn area_struct_ref(rect: &Rectangle) -> u32{
    rect.width * rect.height // now everything is very clear and no ownership issues as well
}