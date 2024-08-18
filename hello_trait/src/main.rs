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
 
// trait as input parameter
fn output_description(d: &impl Describable) {
    println!("{}", d.describe());
}

// trait bounds
fn some_function<T>(t: &T) where T: Describable {
    println!("some as {}", t.describe())
}

// trait bounds
fn some_function2<T: Describable>(t: &T) {
    println!("some2 as {}", t.describe())
}

// trait as return value
fn return_description() -> impl Describable {
    Person  { name: String::from("Tom"), age: 40 }
}

// inherit trait
trait Printable : Describable {
    fn print(&self) {
        format!("{}", self.describe());
    }
}

impl Printable for Person {
    
}

// 
fn main() {
    // test
    let p = Person { name: String::from("John"), age: 30 };
    //println!("{}", p.describe());
    output_description(&p);
    //
    some_function(&p);
    //
    some_function2(&p);
    //
    let p2 = return_description();
    output_description(&p2);
    //
    p.print();/*  */
    //
    println!("Hello, world!");
}
