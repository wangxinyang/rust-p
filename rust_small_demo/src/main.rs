fn main() {
    let test = Test {};
    println!("{}", test.test());
    println!("{}", test.test2());
}

struct Test;

impl Test {
    fn test(&self) -> &str {
        "hello world"
    }

    fn test2<'a>(self) -> &'a str {
        "tosei"
    }
}
