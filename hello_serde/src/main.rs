extern crate serde_json;

use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    is_member: bool,
}

fn json_ds() {
    let p = Person {name: "John".to_string(), age: 30, is_member: true};
    // Serialize the struct to a JSON string
    let json = serde_json::to_string(&p).unwrap();
    println!("serialized = {}", json);
    // Deserialize the JSON string back to a struct
    let p2: Person = serde_json::from_str(&json).unwrap();
    println!("deserialized = {:?}", p2);
}

fn main() {
    json_ds();
    //
    println!("Hello, world!");
}
