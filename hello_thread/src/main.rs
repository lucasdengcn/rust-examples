use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn counter_demo() {
    let count = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    //
    for _ in 0..10 {
        let count = Arc::clone(&count);
        let handle = thread::spawn(move || {
            //
            let mut num = count.lock().unwrap();
            println!("before th{:?}, {:?}", thread::current().id(), num);
            *num += 1;
            println!("after th{:?}, {:?}", thread::current().id(), num);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("{:?}", count.lock().unwrap());
}

fn th_print() {
    let handle = thread::spawn(move || {
        for i in 1..10  {
            println!("thread: {:?}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    //
    for i in 1..5   {
        println!("main thread: {:?}", i);
        thread::sleep(Duration::from_millis(2));
    }
    //
    handle.join().unwrap();
}

fn main() {
    counter_demo();
    th_print();
    println!("Hello, world!");
}
