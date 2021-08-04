use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn run(){
    
    println!("Guess the number!");
    
    // The secret_number type will be determined by the "let guess: u32" statement
    let secret_number = rand::thread_rng().gen_range(1..101);
    
    loop {
    println!("Please input your guess.");
    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // trim method on a String instance will eliminate any whitespace at the beginning and end.
    // The parse method on strings parses a string into some kind of number.
    // The let guess: u32 tell Rust that we want a u32 type from parse an tells the rand the
    // number type we want for comparison.
    
    //let guess: u32 = guess.trim().parse().expect("Please type a number!");
    
    //Ignoring the error
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
        }
    }
}
}