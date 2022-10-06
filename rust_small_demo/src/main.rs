use std::borrow::Borrow;

fn main() {
    let s = String::from("hello");
    let s1: &str = s.borrow();
    let s2: &String = s.borrow();
    println!("s1: {s1:p}, s2: {s2:p}");
    let a = s1.to_owned();
    let b: &str = a.borrow();
    println!("{:?}", b);
}
