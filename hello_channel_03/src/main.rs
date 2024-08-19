use tokio::sync::mpsc;
use std::{thread, time::Duration};

#[tokio::main]
async fn main() {
    //
    let (tx1, mut rx1) = mpsc::channel(10);
    let (tx2, mut rx2) = mpsc::channel(10);
    //
    tokio::spawn(async move {
        for i in 0..10 {
            let v = tx1.send(i+2).await;
            match v {
                Ok(_) => println!("1send out: {:?}", i+2),
                Err(e) => println!("1Failed to send message {:?}", e),
            };
            thread::sleep(Duration::from_millis(1));
        }
    });
    //
    tokio::spawn(async move  {
        for i in 0..10 {
            let v = tx2.send(i+2).await;
            match v {
                Ok(_) => println!("2send out: {:?}", i+2),
                Err(e) => println!("2Failed to send message {:?}", e),
            };
            thread::sleep(Duration::from_millis(2));
        }
    });
    //
    while let val = tokio::select! {
        Some(msg) = rx1.recv() => msg,
        Some(msg) = rx2.recv() => msg,
    } {
        println!("Got val: {:?}", val);
    }
    //
    println!("Hello, world!");
}
