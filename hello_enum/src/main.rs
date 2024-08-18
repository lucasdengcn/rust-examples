enum Color {
    Red,
    Green,
    Blue
}

fn main() {
    let c = Color::Red;
    match c {
        Color::Red => println!("red"),
        Color::Green => println!("green"),
        Color::Blue => println!("blue"),
        _ => println!("not found")
    }
    println!("Hello, world!");
}
