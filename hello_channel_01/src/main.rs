use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[allow(dead_code)]
fn simple(){
    // create a channel with two ends: sender (tx) and receiver (rx).
    let (tx, rx) = mpsc::channel();
    // spawn a thread to send values into the tx.
    let handle = thread::spawn(move || {
        let vals = vec![1,2,3];
        for val in vals {
            tx.send(val).unwrap();
        }
    });
    // receive 1
    let received = rx.recv().unwrap();
    println!("received: {received:?}");
    // receive 2,3, sum = 5
    let sum: i32 = rx.iter().sum();
    println!("sum: {sum:?}");
    handle.join().unwrap();
}

#[allow(dead_code)]
fn ping_pong(){
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    let handle_tx1 = thread::spawn(move || {
        for val in 0..50 {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });
    let handle_rx1 = thread::spawn(move || {
        let mut sum = 0;
        while let Ok(val) = rx1.recv() {
            println!("received in Rx1: {val}");
            tx2.send(val + 100).unwrap();
            sum += val;
        }
        //
        println!("received sum in Rx1: {sum}");
    });
    //
    let handle_rx2 = thread::spawn(move || {
        let mut sum = 0;
        while let Ok(val) = rx2.recv() {
            println!("received in Rx2: {val}");
            sum += val;
        }
        println!("received sum in Rx2: {sum}");
    });
    //
    handle_tx1.join().unwrap();
    handle_rx1.join().unwrap();
    handle_rx2.join().unwrap();
}

fn main() {
    // simple();
    //
    println!("=================");
    ping_pong();
    //
    println!("Hello, world!");
}
