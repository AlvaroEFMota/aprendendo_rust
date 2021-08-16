#[allow(dead_code)]
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

pub fn run() {
    let t = Coin::Quarter(UsState::Alabama);
    value_in_cents(&t);

    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);

    let some_u8_value = 1u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // the _ placeholder cover all the other cases we don't cover, end () means, does nothing.
    }

    // Both code below do the same thing
    let some_u8_value = Some(3u8); // we can use this pattern to cover one case, or ...
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    let some_u8_value = Some(3u8); // ... we can use this second pattern. With this second patter we type less.
    if let Some(3) = some_u8_value {
        println!("three");
    }


    // Both code below do the same thing
    let mut count = 0;
    let coin = Coin::Penny;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    println!("count: {}", count);


    let mut count = 0;
    let coin = Coin::Penny;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    println!("count: {}", count);
}
