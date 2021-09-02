#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    #[allow(dead_code)]
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[allow(dead_code)]
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
    //String::from("Hello!")
}

#[allow(dead_code)]
pub struct Guess {
    value: i32,
}

#[allow(dead_code)]
impl Guess {
    pub fn new(value: i32) -> Guess {
        //if value > 1 {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

     #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");

        // Assert with an message
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    
    // Usando o should_panic
    // if we use expected in should_panic, the message in panic! macro must be the same as expected
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
    

    // We can’t use the #[should_panic] annotation on tests that use Result<T, E>
    #[test]
    fn using_result() -> Result<(), String> {
        //if 2 + 2 == 5 {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}