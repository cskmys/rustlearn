/*
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0]; // throws compilation error coz we don't if "T" datatype moves or copies
    for &item in list{
        if item > largest { // throws compilation error coz we don't if "T" datatype is comparable
            largest = item;
        }
    }
    largest
}
*/

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T { // by specifying the traits we made sure the "T" does a copy and is comparable
    let mut largest = list[0];
    for &item in list{
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_clone<T: PartialOrd + Clone>(list: &[T]) -> T { // specifying clone instead of copy
    // so that data types that work on heap(or implements "move" trait) can also be passed in if they are comparable
    // The trade-off is that cloning takes takes space on heap which brings into picture heap allocation and de-allocation which would incur time
    let mut largest = list[0].clone(); // need to clone coz just assignment will move the clone
    for item in list.into_iter(){ // using iterator to traverse through the list, hence "item" is of type "&T"
        if *item > largest { // de-referencing "item" to do the comparison
            largest = item.clone(); // need to clone coz just de-referencing will move the value
        }
    }
    largest
}

fn main() {
    let nb_list = vec![34, 50, 25, 100, 65];

    let res = largest(&nb_list);
    println!("The largest nb(copy) is {}", res);
    let res = largest_clone(&nb_list);
    println!("The largest nb(clone) is {}", res);

    let ch_list = vec!['y', 'm', 'a', 'q'];

    let res = largest(&ch_list);
    println!("The largest char(copy) is '{}'", res);
    let res = largest_clone(&ch_list);
    println!("The largest char(clone) is '{}'", res);
}