#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let a = 5;
    // let num = Number::from(20);
    // println!("my number is {:?}", num);
    let num: Number = a.into();
    println!("my number is {:?}", num);
}
