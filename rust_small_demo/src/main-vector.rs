#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // let mut v = vec![100, 32, 7];

    // for i in &mut v {
    //     *i += 50;
    // }

    // for i in v {
    //     println!("{}", i)
    // }

    let v = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];

    for i in v {
        println!("{:?}", i)
    }
}
