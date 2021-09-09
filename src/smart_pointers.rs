use std::ops::Deref;
struct MyBox<T>(T); // MyBox is a tuple struct with one element (T)

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn hello(name: &str) {
    println!("hello {}", name);
}

// -------------------------------

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

#[derive(Debug)]
enum List2 {
    Cons(i32, Rc<List2>),
    Nil,
}
// ------------------------------

use crate::smart_pointers::List::{Cons, Nil};
use std::rc::Rc;
use std::mem::drop;

pub fn run() {
    let b = Box::new(5);
    println!("b = {}", b);
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("list: {:?}", list);

    // Using reference
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);
    //assert_eq!(5, y); //Error because y is of the type &{integer}
    println!("x:{}  y:{}  *y:{}", x, y, *y);

    // Using Box<T> like a reference
    let x = 5;
    let y = Box::new(x); // The x value is copied and y is an instance of Box pointing to these copied value
    assert_eq!(x, 5);
    assert_eq!(x, *y);

    // Using MyBox
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(x, *y); // we need to implement deref to use * notation

    let name: MyBox<String> = MyBox::new(String::from("Álvaro"));
    hello(&name); // It works beacuse of deref implementation

    
    // Dropping a Value Early with std::mem::drop
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("@CustomSmartPointer created.");
    drop(c);
    println!("@CustomSmartPointer dropped before the end of main.");

    println!("---------------------");

    // Drop Trait
    let _a = CustomSmartPointer{ data: String::from("first test") };
    let _b = CustomSmartPointer{ data: String::from("second test") };
    println!("CustomSmartPointers created");

    // Rc<T>, the Reference Counted Smart Pointer
    let a = Rc::new(List2::Cons(5, Rc::new(List2::Cons(10, Rc::new(List2::Nil)))));
    let b = Rc::new(List2::Cons(3, Rc::clone(&a))); // Instead of a.clone() we can use Rc::clone(&a)
    let c = Rc::new(List2::Cons(4, a.clone()));
    println!("List b: {:?}", b);
    println!("List c: {:?}", c);


}