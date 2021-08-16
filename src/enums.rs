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

#[derive(Debug)] // Used to show when we use {:?} in println!
enum IpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)] // Used to show when we use {:?} in println!
enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

/* Rust definition of IpAddr (exemple). We can put anything we want inside the enum, even anothe enum.
 struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
*/

enum _Message {
    Quit,
    Move {x: i32, y: i32},
    Write (String),
    ChangeColor (i32, i32, i32),
}

/* Rust implementation of Option enum
 enum Option<T> {
    Some(T),
    None,
 }
 */

pub fn run(){
    let avatar1 = Moviment::Up;
    let avatar2 = Moviment::Down;
    let avatar3 = Moviment::Left;
    let avatar4 = Moviment::Right;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);

    let four = IpAddr::V4(String::from("127.0.0.1"));
    let six = IpAddr::V6(String::from("::1"));
    println!("four: {:?}   six: {:?}", four, six);

    let four = IpAddr2::V4(127,0,0,1);
    let six = IpAddr2::V6(String::from("::1"));
    println!("four: {:?}   six: {:?}", four, six);

    let _some_number = Some(5);
    let _some_string = Some(String::from("Test"));
    let _some_string_literal = Some("Test");
    let _absent_number: Option<i32> = None;
    // Read to be more familiar. https://doc.rust-lang.org/std/option/enum.Option.html
    // Go to match.rs exemple
    
}