use std::collections::HashMap;

pub fn run(){
    
    //Creating a Hash Map
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("scores: {:?}", scores);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value); // map get the ownership of both Strings
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    // Accessing Values in a Hash Map
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(i) => println!("score of Blue: {}", i),
        _ => (),
    }

    // Iterating in a HashMap
    for (key, value) in &scores {
        println!("key: {}  value: {}", key, value);
    }

    // Insert if there are no key value. Only Red and Green will be insert with 50
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(50);
    scores.entry(String::from("Green")).or_insert(50);
    println!("scores: {:?}", scores);

    // Updating a Value Based on the Old Value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    // Using iteration
    let cars = vec![String::from("ModelS"), String::from("Honda")];
    let aceleration = vec![50,15];
    let mut especification: HashMap<_, _> =
        cars.into_iter().zip(aceleration.into_iter()).collect();
    println!("{:?}", especification);

    //Overwriting a value
    especification.insert(String::from("ModelS"), 55);
    println!("{:?}", especification);

}