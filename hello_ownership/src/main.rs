fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    //println!("s1 = {}, s2 = {}", s1, s2);
    //println!("{}", s1);
    println!("{}, len={}", s2, calculate_length(&s2));
    //
    display(s2);
    // println!("{}", s2); // s2 borrowed by display function
    let mut s3 = String::from("hello");
    //
    println!("change {}", change(&mut s3));
}

fn display(s: String) {
    println!("s = {}", s);
}
// function
fn calculate_length(s: &String) -> usize {
    s.len()
}
// function
fn change(s: &mut String) -> &mut String {
    s.push_str(", world");
    s
}