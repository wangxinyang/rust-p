fn main() {
    let x = String::with_capacity(10) + "a";
    let y = String::from("aaaa") + "tosei";
    println!("{}", x);
    println!("{}", y);

    let a = 10;
    let b = &a as *const i32;
    println!("{}", &a); // 10
    println!("{:p}", &a); // 0x7ff7be348d04
    println!("{:?}", b); // 0x7ff7be348d04
}
