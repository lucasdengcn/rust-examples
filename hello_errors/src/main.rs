use std::fs::File;
use std::io::Read;

// custom error type
#[derive(Debug)]
struct MyError {
    message: String,
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyError: {}", self.message)
    }
}

fn do_something() -> Result<(), MyError>  {
    Err(MyError { message: "Something went wrong".to_string() })
}

impl std::error::Error for MyError {}

fn get_data_from_file(filename: &str) -> Result<Vec<u8>, std::io::Error>  {
    // error propagation
    let mut file = File::open(filename)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    Ok(data)
}

fn main() {
    // do_something().unwrap();
    let txt = get_data_from_file("test2.txt").unwrap();
    println!("{}", String::from_utf8(txt).unwrap());
    println!("Hello, world!");
}
