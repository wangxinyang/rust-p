use std::fs;

fn main() {
    let path = "/tmp/dat";
    println!("{}", read_file(path));
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}
