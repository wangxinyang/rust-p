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

    let str = String::from("hello world");
    let _str1 = &str;

    let s = "hello world";
    let res: Vec<&str> = s.splitn(4, |c| c == 'l').collect();
    println!("{:?}", res);
}
