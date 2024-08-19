// generic function
fn display<T: std::fmt::Debug>(x: T) {
    println!("{:?}", x);
}

// generic struct
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn area<T: std::ops::Mul<Output = T> + Clone >(p: &Point<T>) -> T {
    p.x.clone() * p.y.clone()
}

struct Zone<T> {
    x: T,
    y: T,
}

impl<T> Zone<T> {
    //
    fn new(x: T, y: T) -> Self  {
        Self { x, y }
    }
    //
    fn x(&self) -> &T {
        &self.x
    }
}

// generic enum
enum MyOption<T> {
    Some(T),
    None,
}

fn display_options<T: std::fmt::Display>(x: MyOption<T>) {
    match x {
        MyOption::Some(x) => println!("Got: {}", x),
        _ => println!("No value found"),
    }
}

fn main() {
    display(42);
    display([1,2,3]);
    display("Hello, world!");
    //
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 2.0, y: 3.0 };
    //
    println!("{}", area(&p1));
    println!("{}", area(&p2));
    //
    display(&p1);
    display(&p2);
    //
    let x = MyOption::Some(5);
    let y: MyOption<i32> = MyOption::None;
    //
    display_options(x);
    display_options(y);
    //
    let z1 = Zone::new(1, 2);
    println!("z1.x {}", z1.x());
    //
    let z2 = Zone::new("a", "b");
    println!("z2.x {}", z2.x());
}
