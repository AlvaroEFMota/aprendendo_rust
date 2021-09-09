use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};

pub fn run() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap(); // wait the thread finish

    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here is a vector: {:?}", v);
    });
    handle.join().unwrap();

    // Using channels
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("Hi");
        tx.send(val).unwrap();
    });
    let receiver = rx.recv().unwrap();
    println!("Got: {}", receiver);

    //Producer and Consumer
    let (tx, rx) = mpsc::channel();
    let producer = thread::spawn(move || {
        for i in 1..8 {
            thread::sleep(Duration::from_millis(500));
            println!("Produziu {}", i);
            tx.send(i).unwrap();
        }
    });
    let consumer = thread::spawn(move || {
        for i in rx {
            thread::sleep(Duration::from_secs(1));
            println!("Consumiu {}", i);
        }
    });
    producer.join().unwrap();
    consumer.join().unwrap();
    println!("---------------------");

    // Creating Multiple Producers by Cloning the Transmitter
    let (tx1, rx) = mpsc::channel();
    let tx2 = tx1.clone();
    let producer1 = thread::spawn(move || {
        for i in 1..5 {
            thread::sleep(Duration::from_millis(500));
            println!("#Produziu {}", i);
            tx1.send(i).unwrap();
        }
    });
    let producer2 = thread::spawn(move || {
        for i in 5..10 {
            thread::sleep(Duration::from_millis(500));
            println!("$Produziu {}", i);
            tx2.send(i).unwrap();
        }
    });
    let consumer = thread::spawn(move || {
        for i in rx {
            thread::sleep(Duration::from_secs(1));
            println!("Consumiu {}", i);
        }
    });
    producer1.join().unwrap();
    producer2.join().unwrap();
    consumer.join().unwrap();

    // Mutex
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);

    // Sharing a Mutex<T> Between Multiple Threads
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
