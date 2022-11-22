use std::convert::From;

fn main() {
    // let a = String::from("1");
    // let a: String = From::from("hello world");
    // let a = Number::from(16.2);
    // println!("{:?}", a.value);
    // let a = f64::from(32);
    // println!("{:?}", a);
    // let b: i32 = 11i16.into();
    // println!("{:?}", b);
    let mut c = Vec::new();
    c.push("-".to_string());

    let mut d = Vec::new();
    d.push("-");

    let mut e = Vec::new();
    e.push("1");

    let b = Vec::from(String::from("hello world"));
    let mut a: Vec<String> = vec![String::from("-")];
    a.push(String::from("1"));
    a.push(String::from("2"));
    println!("{:?}", a);
}

struct Number {
    value: i32,
}

impl From<f64> for Number {
    fn from(from_value: f64) -> Self {
        Number {
            value: from_value as i32,
        }
    }
}
