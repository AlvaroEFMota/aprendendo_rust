/* Primitive string = Immutable fixed-length (string somewhere in memory)
*  String = Growable, heap-allocated data structure - Use when you need to modify or own
*/

pub fn run() {
    let hello = "Hello"; // Primitive string
    let mut world = String::from("W"); // String
    let name = String::from("Álvaro Ernani Fonseca Mota");
    //Get length
    println!("length of hello = {}", hello.len());

    //push a char
    world.push('o');

    //push a string
    world.push_str("rld!");
    println!("{} {}", hello, world);

    //Capacity in bytes
    println!("capacity of world = {}", world.capacity());

    // Check if empty
    println!("em empty? {}", hello.is_empty());

    // Contains
    println!("world conteins rld? {}", world.contains("rld"));

    //Replace
    println!(
        "replace ld in world with th : {}",
        world.replace("ld", "th")
    );

    //Loop through string by whitespace
    for w in name.split_whitespace() {
        println!("{}", w);
    }

    //Create a string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    //Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);

    {
        let s1 = String::from("hello");
        let s2 = s1; // the stack of s1 was copied to s2, but the heap is the same;
                     //println!("{}, world!", s1); //this piece of code will cause an error, because s1 was "moved" to s2
        println!("s2 = {}", s2);
    }

    {
        let s1 = String::from("hello");
        let s2 = s1.clone(); // s2 has a new stack and a new heap with the content equal to s1
        println!("s1 = {}, s2 = {}", s1, s2);
    }

    //Creating a new empty string
    let mut _s = String::new();

    let data = "initial contents";
    let s = data.to_string();
    println!("value of s: {}", s);

    let hello = String::from("السلام عليكم, Dobrý den, Hello, שָׁלוֹם, नमस्ते, こんにちは, 안녕하세요, 你好, Olá, Здравствуйте, Hola");
    println!("hello: {}", hello);

    let mut s = String::from("lo");
    s.push('l'); // Pushing an character
    println!("s: {}", s);
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used because we call "fn add(self, s: &str) -> String {" from s1
    println!("s3: {}", s3);

    let word = "नमस्ते";
    for c in word.chars() {
        println!("{}", c);
    }
    println!("--------------");
    for c in "नमस्ते".chars() { // Do the same thing as the code above
        println!("{}", c);
    }
    println!("--------------");
    for c in word.as_bytes() {
        println!("{}", c);
    }
}
