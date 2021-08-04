// Reference Pointers -- Point to a resource in memory

pub fn run(){
    // Primitive array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;
    println!("Values: {:?}", (arr1, arr2));

    // With non-primitives, if you assign another variable to a piece of data, the first
    // variable will no longer hold that value. You'll need to use a reference (&) to point to the
    // resource. The second variable will hold that value.

    // Vector
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;
    println!("Values: {:?}", (&vec1, vec2));

    let mut x = 57; //x é mutável
    let a = &mut x; // passando uma referência que permite mudar o valor de x
    *a = 3;
    println!("val x: {}",x);

    // Create some data, a raw pointer pointing to it and a null pointer
    let data: u32 = 42;
    let raw_ptr = &data as *const u32;
    let null_ptr = std::ptr::null() as *const u32;

    // the {:p} mapping shows pointer values as hexadecimal memory addresses
    println!("Data address: {:p}", &data);
    println!("Raw pointer address: {:p}", raw_ptr); 
    println!("Null pointer address: {:p}", null_ptr); 
}