// Generics

struct Point<T> {
    x: T,
    y: T
}

struct Coord<T,U> {
    x: T,
    y: U
}

fn add<T: std::ops::Add<Output=T> + std::ops::Sub<Output=T> + std::fmt::Debug>( a: T, b: T ) -> T {
    println!("[add] a: {:?}", a);
    a+b
}

fn add2<T> ( a: T, b: T ) -> T
    where T: std::ops::Add<Output=T> + std::ops::Sub<Output=T> + std::fmt::Debug {
    println!("[add2] a: {:?}", a);
    a+b
}

fn add3<T,U> ( a: T, b: T, _c: U ) -> T //variables unuserd must start with underscore
    where T: std::ops::Add<Output=T> + std::ops::Sub<Output=T> + std::fmt::Debug,
          U: std::fmt::Debug {
    println!("[add2] a: {:?}", a);
    println!("[add2] b: {:?}", b);
    a+b
}

fn largest<T>(list: &[T]) -> T
    where T: std::cmp::PartialOrd + std::marker::Copy{
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn run(){
    let p1 = Point { x: 54, y: -1025 };
    println!("point: {} {}", p1.x, p1.y);

    let p2 = Point { x: 14.3, y: -23.5 };
    println!("point: {} {}", p2.x, p2.y);

    let c1 = Coord { x: 25, y: 0.02};
    println!("coord: {} {}", c1.x, c1.y);

    let r = add(4.7,0.3);
    let s = add2(2, 7);
    let t = add3(4, 2, 9);
    println!("r: {}", r);
    println!("s: {}", s);
    println!("t: {}", t);

    println!("largest: {}", largest(&[10,15,3,7]));
}