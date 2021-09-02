/**
 * &i32        // a reference
 * &'a i32     // a reference with an explicit lifetime
 * &'a mut i32 // a mutable reference with an explicit lifetime
 *
 **/

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// If we changed the implementation of the longest function to always return
// the first parameter rather than the longest string slice, we wouldn’t need to
// specify a lifetime on the y parameter
fn longestx<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

// Lifetime Annotations in Struct Definitions
struct ImportantExcerpt<'a> {
    part: &'a str,
}


// As we already seen
//fn first_word<'a>(s: &'a str) -> &'a str {

// Generic Type Parameters, Trait Bounds, and Lifetimes Together
use std::fmt::Display;
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

pub fn run() {
    let a = longest("abc", "abcd");
    println!("{}", a);

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    /*
    // Rust knows this because we annotated the lifetimes of the function parameters
    // and return values using the same lifetime parameter 'a.
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str()); //cause an Error
    }
    println!("The longest string is {}", result);
    */

    let a = longestx("abc", "abcdefg");
    println!("{}", a);

    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("part: {}", i.part);


    let a = longest_with_an_announcement("abc", "abcdefg","This is an announcement");
    println!("{}", a);
}
