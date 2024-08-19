use tokio::fs::File;
use tokio::io::AsyncReadExt;
use regex::Regex;

#[tokio::main]
async fn main() {
    let mut file = File::open("example.txt").await.expect("Failed to open file");
    let mut contents = Vec::new();
    //
    file.read_to_end(&mut contents).await.expect("Failed to read the file");
    //
    let s = String::from_utf8(contents).expect("Data not UTF-8");
    // println!("{:?}, {:?}", s, s.len());
    println!("=========");
    s.split('\n').for_each(|line| {
        println!("{}", line);
    });
    //
    println!("=========");
    let regex = Regex::new(r"([a-zA-Z]+)").unwrap();
    for cap in regex.captures_iter(&s) {
        let val = cap.get(0).unwrap();
        println!("{:?}", val);
    }
    //
    println!("Hello, world!");
}
