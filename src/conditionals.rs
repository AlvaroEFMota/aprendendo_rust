// Conditionals -- Used to check the condition of something and act on the result

pub fn run(){
    let age: u8 = 18;
    let check_id: bool = false;


    // IF/ELSE 
    if age >= 21 && check_id {
        println!("Bartender: what would you like to drink");
    } else if age < 21 && check_id {
        println!("Bartender: Sorry, you have to leave");
    } else {
        println!("Bartender: I'll need to see your ID");
    }
    
    //shorthand IF
    let is_of_age = if age >= 21 { true } else { false };
    println!("is_of_age: {}", is_of_age);
}