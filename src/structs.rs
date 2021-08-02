// Structs -- Used tp create custom data types

// Traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

// Tuple Struct
struct ColorT(u8, u8, u8);

//Person struct
struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Construct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    // Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    //Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run(){
    // Traditional struct
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0
    };
    c.red = 200;
    println!("Color: {} {} {}", c.red, c.green, c.blue);

    //Tuple struct
    let mut c1 = ColorT(255,0,0);
    c1.1 = 35;
    println!("ColorT: {} {} {}", c1.0, c1.1, c1.2);

    // Using person struct
    let mut p = Person::new("John", "Doe");
    println!("Person: {}", p.full_name());
    p.set_last_name("Williams");
    println!("Person: {}", p.full_name());
    println!("Person tuple: {:?}", p.to_tuple());


}