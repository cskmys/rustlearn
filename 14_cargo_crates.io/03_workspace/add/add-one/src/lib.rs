use rand;

pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn add_rand(x: i32) -> i32 {
    x + rand::random::<i32>()
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn it_works(){
        assert_eq!(3, add_one(2));
    }
}