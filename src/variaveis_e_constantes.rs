pub fn run(){
    let gravit = 9.8; // without mut, gravit is inmutable, we can't assign other value
    println!("gravit force is {}", gravit);
    
    let mut speed = 50;
    println!("before the speed is {}",speed);
    speed = 42;
    println!("now the speed is {}",speed);

    const PI: f64 = 3.14159265; //we can use const values, but it's not common
    const ID: i32 = 001; // we need especify the type in const
    println!("PI = {}, ID = {}", PI, ID);

    let (sky_color, human_max_speed) = ("blue", 35); //assign multiples variables
    println!("the sky color is {} and the human maximum speed is {}", sky_color, human_max_speed);

    //Assign multiple variables at once
    let ( my_name, my_age) = ("Álvaro", 25);
    println!("my name is {} and my age is {}", my_name, my_age);

    // Suposse that an user input a string with whitespaces
    let spaces = String::from("    ");
    // We would like thar spaces have the number of whitespaces without being mutable (We'll use shadowing)
    let spaces = spaces.len();
    println!("spaces: {}", spaces);
}