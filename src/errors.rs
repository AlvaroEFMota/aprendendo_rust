/**
 * Error handling
 *
 * Add this code to cargo.toml file to change from Unwinding to abort
 *
 * [profile.release]
 * panic = 'abort'
 *
 *
 * enum Result<T, E> {
 *    Ok(T),
 *    Err(E),
 * }
 **/
use std::fs::File;
#[allow(unused_imports)]
use std::io::ErrorKind;
use std::io::Read;
use std::io;
use std::fs;


// Propagating Errors
#[allow(dead_code)]
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// A Shortcut for Propagating Errors: the ? Operator
#[allow(dead_code)]
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// Another shortcut
#[allow(dead_code)]
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// More shorter
#[allow(dead_code)]
fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

pub fn run() {
    //panic!("crash and burn"); // This code will cause a panic

    let _v = vec![1, 2, 3];
    // _v[99];  // panic!, to see the backtrace, type RUST_BACKTRACE=1 cargo run

    //let _f: u32 = File::open("hello.txt"); //Error



    //let f = File::open("hello.txt"); //uncomment this
    /*match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };*/

    /*
    // Matching on Different Errors
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    // Or we can use a better way
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    */



    // Shortcuts for Panic on Error
    // if the file exist, f will be that file, else, the program will panic
    //let _f = File::open("hello.txt").unwrap();

    //Show the panic message inside expect()
    //let _f = File::open("hello.txt").expect("Failed to open hello.txt");

    //let username = read_username_from_file();
    //let username = read_username_from_file2();
    //let username = read_username_from_file3();
    let username = read_username_from_file4();
    let username = match username {
        Ok(un) => un,
        Err(error) => panic!("Não tem username aqui: {}", error),
    };
    println!("username: {}", username); //create a file hello.txt outside of src with a text inside and try.
}
