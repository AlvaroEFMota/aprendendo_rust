

pub fn run(){
    // String Literals Are Slices: the car_model is an slice because its type is &str that gether
    // the pointer value and the lenght.
    let car_model = "Tesla XYZ";
    let car = &car_model[0..5];
    println!("car: {}", car);

    let s = String::from("hello world"); //we can't have unicode here to use the slices.
    //let hello = &s[0..5];
    //let world = &s[6..11]; // The same as below
    let hello = &s[..5];
    let world = &s[6..];
    println!("{} {}!", hello, world);
    
    let mut w2 = String::from(world); //w2 is independent
    w2.push_str("!@#$!");
    println!("w2: {}",w2);


    let fw = first_word(&s);
    println!("first word: {}", fw); //fw is an slice of s;

    let fw = first_word_gen(&s);
    println!("first word gen: {}", fw); //fw is an slice of s;

    let sw = second_word(&s);
    println!("second word: {}", sw);

    let a = [1, 2, 3, 4, 5]; // Slice of the type &[i32]. It works the same way as string slices
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

// we have to pass &String because bytes will have a pointer to &String
// if we pass String instead of &String, s will have the ownnership and will finish at the
// end of the scope, making bytes, &s[..i] and &s[..] be invalid.
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn first_word_gen(s: &str) -> &str { //Little change
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn second_word(s: &String) -> &str { // [TODO] Create error return 
    let bytes = s.as_bytes();
    let _len = s.len();
    let mut b_first: bool = false;
    let mut first: usize = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' && b_first {
            return &s[first+1..i];
        } else if item == b' ' && !b_first {
            first = i;
            b_first = true;
        }
    }
    if b_first {
        return &s[first+1..]
    }
    &s[..] 
}