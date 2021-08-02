pub fn run() {
    let args: Vec<String> = std::env::args().collect();
    let command = args[1].clone();
    println!("Args: {:?}", args);
    println!("Command: {}", command);

    if command == "alvaro" {
        println!("Hi alvaro, how are you?");
    }
}