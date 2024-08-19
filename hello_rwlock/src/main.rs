//
use std::sync::{Arc, RwLock};
use std::thread;
//
fn main() {
    let a = Arc::new(RwLock::new(5));
    // create reader thread
    let reader = Arc::clone(&a);
    let reader_thread = thread::spawn(move || {
        let num = reader.read().unwrap();
        println!("{:?}", *num);
    });
    // create writer thread
    let writer = Arc::clone(&a);
    let writer_thread = thread::spawn(move || {
        let mut num = writer.write().unwrap();
        *num += 1;
    });
    //
    reader_thread.join().unwrap();
    writer_thread.join().unwrap();
    //
    println!("Final data: {}", a.read().unwrap());
    println!("Hello, world!");
}
