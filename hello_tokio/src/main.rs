use reqwest;

#[tokio::main]
async fn entry(){
    let resp = reqwest::get("https://httpbin.org/ip").await.unwrap();
    let body = resp.text().await.unwrap();
    println!("{body}");
}

fn main() {
    entry();
    println!("Hello, world!");
}

// use std::collections::HashMap;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let resp = reqwest::get("https://httpbin.org/ip2")
//         .await?
//         .json::<HashMap<String, String>>()
//         .await?;
//     println!("{resp:#?}");
//     Ok(())
// }