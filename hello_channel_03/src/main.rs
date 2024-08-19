use tokio::sync::mpsc;
use std::{thread, time::Duration};
use tokio::time::sleep;

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
    // while let val = tokio::select! {
    //     Some(msg) = rx1.recv() => msg,
    //     Some(msg) = rx2.recv() => msg,
    // } {
    //     println!("Got val: {:?}", val);
    // }

    // 等待并处理来自两个通道的数据  
    loop {  
        tokio::select! {  
            msg1 = rx1.recv() => {  
                if let Some(msg) = msg1 {  
                    println!("Received from channel 1: {}", msg);  
                } else {  
                    // break; // 假设通道关闭，退出循环  
                }  
            },  
            msg2 = rx2.recv() => {  
                if let Some(msg) = msg2 {  
                    println!("Received from channel 2: {}", msg);  
                } else {  
                    // break; // 假设通道关闭，退出循环  
                }  
            },  
            _ = sleep(Duration::from_millis(3)) => {  
                println!("No messages received for 1 second, checking again...");  
            }  
        }  
    }  

    //
    println!("Hello, world!");
}
