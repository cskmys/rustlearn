#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32
}
impl Rectangle{ // to define a method associated with "Rectangle" type
    fn area(self: &Self) -> u32{ // previously area_struct_ref took "rect: &Rectangle" as parameter, now we replace it with "self: &Self" or its shorthand "&self"
        // to avoid losing ownership, "&Self" is used instead of "Self"
        self.width * self.height
    }
    fn width(&self) -> bool{
        Rectangle::val_chk(self.width) // calling associated function using "<struct>::<method>(<param>)" syntax
    }
    fn height(&self) -> bool{
        Rectangle::val_chk(self.height)
    }
    fn area_checking(&self) -> u32{
        if self.width() && self.height(){
            return self.area();
        }
        println!("Invalid dimensions"); // now we have additional print msg
        0
    }

    fn can_hold(&self, rect: &Rectangle) -> bool { // can the "rect" fit within the current Rectangle
        self.width >= rect.width && self.height >= rect.height
    }
}

impl Rectangle { // multiple "impl" blocks can be used
    fn val_chk(nb: u32) -> bool{ // this doesn't need an instance of struct to be called, hence it doesn't have "self", "&self" or "&mut self" as the first parameter
        nb > 0
    }
    fn square(siz: u32) -> Rectangle{
        Rectangle{
            width: siz,
            height: siz
        }
    }
}

fn main() {
    let width = 30;
    let height = 50;

    let rect = Rectangle{
        width,
        height
    };
    println!("Area of rectangle is {} square pixels", rect.area());

    let square = Rectangle::square(0);
    println!("Area of square(without any param validation) is {} square pixels", square.area());
    println!("Area of square(with any param validation) is {} square pixels", square.area_checking());

    let rect1 = Rectangle{
        width: width * 2,
        height
    };
    let rect2 = Rectangle{
        width,
        height
    };
    println!("rect can hold rect1? = {}", rect.can_hold(&rect1));
    println!("rect can hold rect2? = {}", rect.can_hold(&rect2));
}
