// Arrays -- Fixed list where elements are the same data types

use std::mem; //if we put this, we can only use mem::size_of_val() intead of std::mem::size_of_val()

pub fn run(){
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    

    // Re-assign value
    numbers[2] = 20;

    println!("{:?}", numbers);

    // Get single value
    println!("single val: {}", numbers[0]);

    // Get array length
    println!("Array lenth {}", numbers.len());

    //Arrays are stack allocated
    println!("This array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers;
    let slice1: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
    println!("Slice1: {:?}", slice1);

    let ar = [3; 5];
    println!("ar: {:?}", ar);
}