fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list{
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list{
        if item > largest {
            largest = item;
        }
    }
    largest
}

/*
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0]; // throws compilation error
    for &item in list{
        if item > largest { // throws compilation error
            largest = item;
        }
    }
    largest
}
// error due to not having defined traits
// will do that in the later sub-chapter
// for now just know that both "largest_i32" and "largest_char" methods can be replaced by one function
*/

#[derive(Debug)]
struct Point<T>{
    x: T,
    y: T
}

impl<T> Point<T>{ // "impl<T>" indicates that this implementation is applicable for generic data type "T"
fn x(&self) -> &T {
    &self.x
}
}

impl Point<f32>{ // available specifically for struct of type "Point<f32>", hence just has "impl" instead of "impl<T>"
fn dist_from_origin(&self) -> f32 {
    (self.x.powi(2) + self.y.powi(2)).sqrt()
}
}

#[derive(Debug)]
struct PointDiff<T, U>{
    x: T,
    y: U
}

impl<T, U> PointDiff<T, U>{
    fn mixup<V, W>(self, other: PointDiff<V, W>) -> PointDiff<T, W> { // data types "V, W" other than the ones belonging to struct field "T, U" are used in method
        PointDiff {
            x: self.x,
            y: other.y
        }
    }
}

fn main() {
    let nb_list = vec![34, 50, 25, 100, 65];

    let res = largest_i32(&nb_list);
    println!("The largest nb is {}", res);

    let ch_list = vec!['y', 'm', 'a', 'q'];

    let res = largest_char(&ch_list);
    println!("The largest char is '{}'", res);

    let integer = Point {x: 5, y: 10};
    println!("point<i32> = {:?}, p.x = {}", integer, integer.x());
    let float = Point {x: 5.0f32, y: 10.0};
    let dist = float.dist_from_origin();
    println!("point<f32> = {:?}, p.x = {}, distance from origin is {}", float, float.x(), dist);
    /*
    let dist = integer.dist_from_origin(); // throws compilation error
    method "dist_from_origin" is defined only for "f32" data type not "i32" as well.
    */

    /*
    let wont_work = Point{x: 5, y:10.0}; // throws compilation error
    // type inference encounters "5" first and expects struct to be of type "Point<i32>"
    // then when it encounters "10.0" which is not "i32", it throws an error
    */
    let now_works = PointDiff{x: 5, y:10.0f32};
    let another = PointDiff{x: "Hello", y:'c'};
    println!("pointdiff1<i32, f32> = {:?}, pointdiff2<&str, char> = {:?}", now_works, another);
    let mixed = now_works.mixup(another);
    println!("mixed<i32, char> = {:?}", mixed);
}