use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};
type Result<T> = std::result::Result<T, DoubleError>;

fn main() {
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["212", "93", "18"];

    print(double_first(numbers));
    print(double_last(empty));
    print(double_last(strings));
}

#[derive(Debug, Clone)]
struct DoubleError;

impl Display for DoubleError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "invalid item to double")
    }
}

impl Error for DoubleError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        // 把错误换成我们的新类型。
        .ok_or(DoubleError)
        .and_then(|s| {
            s.parse::<i32>()
                // 这里也换成新类型。
                .map_err(|_| DoubleError)
                .map(|i| 2 * i)
        })
}

fn double_last(vec: Vec<&str>) -> Result<i32> {
    vec.last()
        .ok_or_else(|| DoubleError)
        .and_then(|s| s.parse::<i32>().map_err(|_| DoubleError).map(|i| i * 3))
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}
