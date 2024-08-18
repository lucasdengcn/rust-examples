
//#[derive(std::fmt::Display)]
struct Rectangle {
    width: u32,
    height: u32,
}
// 方法
impl Rectangle {
    // method
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // method
    fn is_square(&self) -> bool {
        self.width == self.height
    }
    //
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    //
    println!("rect area: {}", rect.area());
    println!("rect is square: {}", rect.is_square());
    //
    println!("Hello, world!");
}
