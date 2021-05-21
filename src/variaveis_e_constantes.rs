pub fn run(){
    let gravit = 9.8; // without mut, gravit is inmutable, we can't assign other value
    println!("gravit force is {}", gravit);
    
    let mut speed = 50;
    println!("before the speed is {}",speed);
    speed = 42;
    println!("now the speed is {}",speed);

    const PI: f64 = 3.14159265; //we can use const values, but it's not common
    println!("PI = {}", PI);

    let (sky_color, human_max_speed) = ("blue", 35); //assign multiples variables
    println!("the sky color is {} and the human maximum speed is {}", sky_color, human_max_speed);
}