/* Tuples group together values of different types
*   Max 12 elements
*/

fn area(dimensions: (i32, i32)) -> i32 {
    dimensions.0*dimensions.1
}

pub fn run(){
    let person: (&str, &str, i8) = ("Álvaro","Ernani", 25);
    println!("{} {} idade: {}", person.0, person.1, person.2);

    let rectangle = (20,20);
    let rect_area = area(rectangle);
    println!("Rectangle area: {}", rect_area);
}