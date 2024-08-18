use std::collections::HashMap;

fn main() {
    let mut hm  = HashMap::new();
    hm.insert(10, "ten");
    hm.insert(20, "twenty");

    let value = hm.get(&30).unwrap_or(&"not found");
    println!("{}", value);

    hm.get(&10).map(|v| println!("{}", v));

    if let Some(v) = hm.get(&20) {
        println!("found: {}", v);
    } else {
        println!("not found");
    };

    let error_msg = String::from("not found");
    let value = hm.remove(&10).unwrap_or(&error_msg);
    println!("{}", value);
    //
    hm.insert(30, "3*ten");
    for (k, v) in &hm {
        println!("{}, {}", k, v);
    }
    //
    println!("Hello, world!");
}
