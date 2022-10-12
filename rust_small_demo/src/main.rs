use std::error::Error;

///读取文件内容
fn read_file(path: &str) -> std::result::Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}

/// 转换为utf8内容
fn to_utf8(v: &[u8]) -> std::result::Result<&str, std::str::Utf8Error> {
    std::str::from_utf8(v)
}

/// 转化为u32数字
fn to_u32(v: &str) -> std::result::Result<u32, std::num::ParseIntError> {
    v.parse::<u32>()
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let path = "./dat";
    let v = read_file(path)?;
    let x = to_utf8(v.as_bytes())?;
    let u = to_u32(x)?;
    println!("num:{:?}", u);
    Ok(())
}
