pub fn run(){
    //Default is "i32"
    let x = 1;
    
    //Default is "f64"
    let y = 0.5;

    //Add explicit type
    let z: i64 = 425789965544236547;

    //Find max size
    println!("the max value of an i32 is {}", std::i32::MAX);
    println!("the max value of an i64 is {}", std::i64::MAX);

    //Boolean
    let is_active = true;
    let is_greater = 10 > 1;

    //Character
    let a1 = 'Z';
    let jigsaw = '\u{2622}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, jigsaw))

}