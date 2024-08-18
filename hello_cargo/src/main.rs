
fn main() {
    let x = 5;
    let y = 6.0;
    let z = "hello";
    let a = true;
    let b = [1, 2, 3, 4, 5, 6];
    let c = (4,5,6);
    println!("x = {}, y = {}, z = {}, a = {}, b = {:?}, c = {:?}", x,y,z,a,b,c);
    println!("Hello, world!");
    println!("first of b is {}", b[0]);
    let (c1, c2, c3) = c;
    println!("c1 = {}, c2 = {}, c3 = {}", c1, c2, c3);
    let mut mx = 5;
    println!("mx = {}", mx);
    mx += 10;
    println!("mx = {}", mx);
    //
    let mut v2 = vec![1,2,3];
    v2.push(4);
    println!("{:?}",v2);
    //
    let mut s = String::from("hello");
    s.push_str(", world!"); 
    println!("s is {}", s);
    //
    let s3 = &s[0..3];
    println!("s3 = {}", s3);
    //
    let b4 = &b[0..4];
    println!("b4 = {:?}", b4);
    //
    for item in b4.iter() {
        println!("b4 item is {}", item);
    }
    //
    for c in s3.chars() {
        println!("c = {}", c);
    }
    //
    for v in v2.iter() {
        println!("v = {}", v);
    }
}
