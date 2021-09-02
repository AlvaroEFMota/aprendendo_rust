use std::collections::HashMap;
use std::thread;
use std::time::Duration;

// This Cacher struct works like dinamic programming
struct Cacher<T>
where
    T: Fn(u32) -> u32, // T is a closure using the fn trait. The closure must have u32 parameter and return u32
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        if self.value.contains_key(&arg) {
            self.value[&arg]
        } else {
            let v = (self.calculation)(arg);
            self.value.insert(arg, v);
            v
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

pub fn run() {
    let simulated_user_especified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_especified_value, simulated_random_number);

    let mut cache = Cacher::new(|x| {
        println!("Wait a second: processing {}", x);
        thread::sleep(Duration::from_secs(1));
        x + 1
    });

    cache.value(1);
    cache.value(1);
    cache.value(1);
    cache.value(1);
    cache.value(2);
    cache.value(3);
    cache.value(3);
    cache.value(3);
    cache.value(1);
    cache.value(4);

    assert_eq!(cache.value(1), 2);
    assert_eq!(cache.value(4), 5);

    // Capturing the environment
    let x = 4;
    let equal_to_x = |z| z == x; // ownership of x is moved to the closure
    let y = 4;
    assert!(equal_to_x(y));

    // Usong move keword
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    //println!("can't use x here: {:?}", x); // Error: we can't use x here, because x was moved to the closure 
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}
