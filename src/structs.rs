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
    fn to_tuple(&self) -> (String, String) {
        (self.first_name.to_string(), self.last_name.to_string())
    }
}

struct User {
    username: String,
    email: String,
    sing_in_count: u64,
    active: bool,
}



fn build_user(username: String, email: String) -> User {
    User {
        username,           //username: username
        email,              //email: email
        sing_in_count: 1,
        active: true,
    }
}

struct PointST(i32, i32, i32); //struct Tuple
struct ColorST(i32, i32, i32);

#[derive(Debug)] // Used to show when we use {:?} in println!
struct Rectangle {
    width: i32,
    height: i32,
}

fn area(rectangle: &Rectangle) -> i32 {
    rectangle.width*rectangle.height
}

impl Rectangle {
    fn area(&self) -> i32 { // The &self parameter is like &Rectangle parameter workin almost like the area function above
        self.width * self.height
    }

    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        if rectangle.width < self.width && rectangle.height < self.height {
            return true
        }
        false
    }

    fn square(size: i32) -> Rectangle { // We are passing the Rectangle and the ownership as a return 
        Rectangle{
            width: size,
            height: size,
        }
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


    let mut user = User{
        username: String::from("Álvaro E. F. Mota"),
        email: String::from("alvaro11318@gmail.com"),
        sing_in_count: 1,
        active: true,
    };

    println!("user: [{}], [{}], [{}], [{}]", user.username, user.email, user.sing_in_count,user.active);
    user.username = String::from("Álvaro Ernani Fonseca Mota");
    println!("user: [{}], [{}], [{}], [{}]", user.username, user.email, user.sing_in_count,user.active);

    let user1 = build_user(String::from("Laplace"), String::from("laplace@gmail.com"));
    println!("user1: [{}], [{}], [{}], [{}]", user1.username, user1.email, user1.sing_in_count,user1.active);

    let user2 = User {
        username: String::from("Laplace Lisert"),
        ..user1
    };
    println!("user2: [{}], [{}], [{}], [{}]", user2.username, user2.email, user2.sing_in_count,user2.active);


    let black = ColorST(0, 0, 0);// The type Color_st is different from Point_st
    let origin = PointST(0, 0, 0);
    println!("black: {},{},{}   origin: {},{},{}", black.0, black.1, black.2, origin.0, origin.1, origin.2);

    let rect = Rectangle{
        width: 8,
        height: 20,
    };
    let rect_area = area(&rect);
    println!("1) rect: {:?}", rect); // Only avaliable when use the #[derive(Debug)] above Rectangle struct
    println!("2) rect: {:#?}", rect); 
    println!("rect_area: {}", rect_area);
    println!("method area: {}", rect.area()); // Calling the method area of Rectangle [5-13]
    let rect2 = Rectangle{
        width: 10,
        height: 25,
    };
    println!("can_hold?: {}", rect2.can_hold(&rect)); // [5-15]

    let rect3 = Rectangle::square(10);
    println!("square rectangle: {:?}", rect3);
}