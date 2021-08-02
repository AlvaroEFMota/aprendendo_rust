// Enums are types which have a few definite values

enum Moviment {
    // Variants
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: Moviment) {
    // Perform action depending on info
    match m {
        Moviment::Up => println!("Avatar moving Up"),
        Moviment::Down => println!("Avatar moving Down"),
        Moviment::Left => println!("Avatar moving Left"),
        Moviment::Right => println!("Avatar moving Right")
    }
}

pub fn run(){
    let avatar1 = Moviment::Up;
    let avatar2 = Moviment::Down;
    let avatar3 = Moviment::Left;
    let avatar4 = Moviment::Right;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}