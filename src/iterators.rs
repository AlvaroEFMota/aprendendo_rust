#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

// Using Closures that Capture Their Environment
#[allow(dead_code)]
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

// Creating Our Own Iterators with the Iterator Trait
struct Counter {
    count: u32, // private atribute
}

impl Counter {
    fn new() -> Counter {
        // Enforce the behavior of always starting new instances with value 0.
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

pub fn run() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    println!("{:?}", v1_iter);
    for i in v1_iter {
        println!("iter: {}", i);
    }

    // Methods that consume iterators
    let sum: i32 = v1.iter().sum();
    println!("the sum is {:?}", sum);

    // Method that produce other iterators
    let double_iter = v1.iter().map(|x| x * x);
    for i in double_iter {
        println!("double: {}", i);
    }

    // The Iterator Trait and the next Method
    let v2 = vec![1, 2, 3];
    let mut v2_iter = v2.iter();
    assert_eq!(v2_iter.next(), Some(&1));
    assert_eq!(v2_iter.next(), Some(&2));
    assert_eq!(v2_iter.next(), Some(&3));
    assert_eq!(v2_iter.next(), None);

    // Collect method - Turn the iterator into a collection data type
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);

    // Some exemples
    let v3 = vec![1, 2, 3, 4, 5];
    let iter = v3.iter();
    println!("-----");
    for i in iter {
        println!("i: {:?}", i);
    }
    println!("-----");

    let j = v3
        .iter()
        .zip(v3.iter().skip(1));
    for i in j {
        println!("i: {:?}", i);
    }
    println!("-----");
    let j = v3
        .iter()
        .zip(v3.iter().skip(1))
        .map(|(a, b)| a * b);
    for i in j {
        println!("i: {:?}", i);
    }
    println!("-----");
    let j = v3
        .iter()
        .zip(v3.iter().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0);
    for i in j {
        println!("i: {:?}", i);
    }
    println!("-----");
    let sum: u32 = v3
        .iter()
        .zip(v3.iter().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    println!("sum: {}", sum);
    println!("-----");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }

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
    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(18, sum);
    }
}
