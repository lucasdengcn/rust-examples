use tokio::sync::mpsc;
use std::{thread, time::Duration};
// multi consumer channel


#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(100);
    //
    let tx1 = tx.clone();
    tokio::spawn(async move {
        for i in 0..10 {
            let v = tx1.send(i+2).await;
            match v {
                Ok(_) => println!("1send out: {:?}", i+2),
                Err(e) => println!("Failed to send message {:?}", e),
            };
            thread::sleep(Duration::from_millis(1));
        }
    });
    let tx2 = tx.clone();
    tokio::spawn(async move {
        for i in 0..10 {
            let v = tx2.send(i+1).await;
            match v {
                Ok(_) => println!("2send out: {:?}", i+1),
                Err(e) => println!("Failed to send message {:?}", e),
            };
            thread::sleep(Duration::from_millis(2));
        }
    });
    //
    while let Some(msg) = rx.recv().await {
        println!("Got {:?}", msg)
    }
    //
    println!("Hello, world!");
}
