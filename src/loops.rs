pub fn run(){
    let mut count = 0;

    // Infinite loop
    loop{
        count += 1;
        println!("Number {}", count);

        if count == 20 {
            break;
        }
    }

    //While loop (FizzBuzz)
    count = 0;
    while count <= 100 {
        if count % 15 == 0 {
            print!("fizzbuzz ")
        } else if count % 3 == 0 {
            print!("fizz ");
        } else if count % 5 == 0 {
            print!("buzz ");
        }
        print!("{} ", count);

        //Inc
        count += 1;
    }

    println!();
    println!();

    //For range
    for x in 0..100 {
        if x % 15 == 0 {
            print!("fizzbuzz ")
        } else if x % 3 == 0 {
            print!("fizz ");
        } else if x % 5 == 0 {
            print!("buzz ");
        }
        print!("{} ", x);
    }

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Result is {}", result);

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}