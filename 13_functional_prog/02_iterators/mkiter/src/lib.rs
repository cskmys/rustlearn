
#[cfg(test)]
mod tests {
    use crate::Counter;

    #[test]
    fn calling_next_directly() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn skip_fn(){
        let a = [4, 5, 6];

        let mut iter = a.iter().skip(0);  // the "next" iterator will skip 0 elements from the start
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&6));
        assert_eq!(iter.next(), None);

        let mut iter = a.iter().skip(1); // the "next" iterator will skip 1 element from the start
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&6));
        assert_eq!(iter.next(), None);

        let mut iter = a.iter().skip(2); // the "next" iterator will skip 2 elements from the start
        assert_eq!(iter.next(), Some(&6));
        assert_eq!(iter.next(), None);

        let mut iter = a.iter().skip(3); // the "next" iterator will skip 3 elements from the start
        assert_eq!(iter.next(), None);

        let mut iter = a.iter().skip(4); // the "next" iterator will skip 4 elements from the start
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new() // "Counter" with "count" = 0, it's "next" will produce Some(0), Some(1), Some(2), Some(3), Some(4), Some(5), None
            .zip(Counter::new().skip(1)) // "next" will skip 1 element from the start, hence, "count" = 0 is skipped, and it produces Some(1), Some(2), Some(3), Some(4), Some(5), None
            // "zip" will combine two collections element by element as a tuple
            // Pairing the "next" values of both gives (Some(0), Some(1)), (Some(1), Some(2)), (Some(2), Some(3)), (Some(3), Some(4)), (Some(4), Some(5)), (Some(5), "None"), ("None", "None")
            // But "zip" produces output only if both are not "None", hence it gives (0, 1), (1, 2), (2, 3), (3, 4), (4, 5)
            .map(|(a, b)| a * b) // "map" will output 1, 2, 6, 12, 20
            .filter(|x| x % 3 == 0) // "filter" will let only 6, 12
            .sum(); // "sum" will add to produce 18
        assert_eq!(18, sum);
    }
}

struct Counter {
    count: u32
}

impl Counter {
    fn new() -> Counter {
        Counter {
            count: 0 // private coz we want to restrict it to [0, 5]
        }
    }
}

impl Iterator for Counter {
    type Item = u32; // setting the associated type of iterator, meaning the iterator will return values of type "u32"

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}