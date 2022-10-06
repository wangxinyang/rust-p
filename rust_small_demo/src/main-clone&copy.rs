fn main() {
    let a = A {
        // a: vec![1, 2, 3, 4],
        a: 32,
    };
    // clone需要显示调用
    // let b = a.clone();
    let b = a;
    println!("{:?}", a.a);
    println!("{:?}", b.a);
    println!("{:p}", &a);
    println!("{:p}", &b);
}

// Copy需要struct中每个项目都是实现了Copy这个trait的
#[derive(Debug, Copy, Clone)]
// struct A {
//     a: Vec<u8>,
// }
struct A {
    a: i32,
}

// impl Clone for A {
//     fn clone(&self) -> Self {
//         Self {
//             a: vec![1, 2, 3, 4],
//         }
//     }
// }
