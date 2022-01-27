
#[test]
fn iterator_demo(){
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter(); // to use "next" method you need to declare it as "mut" coz "next" method changes the internal state of iterator

    assert_eq!(v1_iter.next(), Some(&1)); // "next" outputs immutable reference to data(wrapped in "Some")
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum(){
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total:i32 = v1_iter.sum(); // "sum" requires type annotation
    assert_eq!(total, 6);

    /*
    let mut v1_iter = v1.iter(); // "mut" coz "next" method will change its internal state
    let total:i32 = v1_iter.sum(); // "sum" consumes "v1_iter" meaning "v1_iter" loses its ownership is no longer valid
    assert_eq!(v1_iter.next(), None); // throws compilation error
    // "v1_iter" is consumed by "sum"
    */
}

#[test]
fn iterator_map() {
    let v1 = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| { x + 1}).collect(); // "collect" can return many types, so you'll need to annotate.
    // but here just a partial annotation saying its a vector is good enough, the compiler will infer the vector's concrete data type
    assert_eq!(v2, vec![2, 3, 4]);
}


#[derive(Clone, PartialEq, Debug)]
struct Shoe{
    size: u32,
    style: String
}

fn shoes_in_size(shoes: &Vec<Shoe>, shoe_size: u32) -> Vec<Shoe>{
    shoes.iter().cloned().collect::<Vec<_>>().into_iter().filter(|s| s.size == shoe_size).collect::<Vec<_>>() // capturing "shoe_size" variable from environment in closure inside "filter"
    // "filter" adapts the given iterator to a new iterator that only contains elements for which the closure returns "true".
    // using "iter" or "iter_mut" to iterate would yield "Option<&Shoe>" or "Option<&mut Shoe>", hence when we do collect, we'll get "Vec<&Shoe>" or "Vec<&mut Shoe>"
    // we need to collect "Vec<Shoe>", hence we iterate using "into_iter" which returns "Option<Shoe>"
    // using "into_iter" causes loss of ownership, hence we need to clone the vector before using "into_iter"
    // "cloned().collect()" gives a clone of the vector and the data type for collect needs to annotated using the "collect::<datatype>()" syntax
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn filters_by_size(){
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 15,
                style: String::from("sandal")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ];
        let shoe_size = 10;
        let in_my_size = shoes_in_size(&shoes, shoe_size);
        println!("Shoes:{:?}", shoes);
        println!("In my size({}):{:?}", shoe_size, in_my_size);
        assert_eq!(in_my_size, vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            }
        ]);
    }
}

fn main() {
    let mut v = vec![1, 2, 3]; // need to declare as "mut" to use "iter_mut" regardless of whether you actually mutate or not
    let v1_iter = v.iter(); // calling "Vec<T>.iter()" creates an iterator but does nothing until it is used by a method i.e. rust iterators are lazy
    for val in v1_iter { // no need to declare "v1_iter" as "mut" coz behind the scenes "for" took ownership of "v1_iter" and made it mutable
        println!("Got(iter): {}", val); // "val" is an immutable reference
    }
    println!("after iter: {:?}", v);

    for val in v.iter_mut() {
        *val = *val * 2; // "val" is a mutable reference
        println!("Got(iter_mut): {}", val);
    }
    println!("after iter_mut: {:?}", v);

    for val in v.into_iter() { // calling "Vec<T>.int_iter()" creates an iterator that returns "Option<T>" when iterated, but it also moves the ownership as well
        println!("Got(into_iter): {}", val); // "val" is value not a reference
    } // "v1" no longer owns the vector
    /*
    println!("{:?}", v1); // throws compilation error
    // "v1" lost ownership and is no longer valid
    */

    let v = vec![1, 2, 3];
    println!("v: {:?}", v);
    let v: Vec<_> = v.iter().map(|x| x + 1).collect(); // as iterators are lazy, just using an iterator adaptor "map" isn't enough, you'll need to use consuming adaptor "collect"
    println!("after map(x + 1) on iter: {:?}", v);

}