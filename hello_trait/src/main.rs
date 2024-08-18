// define trait
trait Describable {
    fn describe(&self) -> String;

    #[allow(dead_code)]
    fn default_description(&self) -> String {
        String::from("This is a default function")
    }
}
// define struct
struct Person {
    name: String,
    age: u8
}

// implement trait for struct
impl Describable for Person {
    fn describe(&self) -> String {
        format!("{} is {} years old", self.name, self.age)
    }
}

fn main() {
    // test
    let p = Person { name: String::from("John"), age: 30 };
    println!("{}", p.describe());
    //
    println!("Hello, world!");
}
