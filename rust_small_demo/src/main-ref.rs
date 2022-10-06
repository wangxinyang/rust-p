use std::{collections::HashMap, ops::Deref};

#[derive(Debug, Clone)]
struct MyData {
    value: i32,
    value2: String,
}

fn demo3() {
    let a = MyData {
        value: 32,
        value2: String::from("11"),
    };
    // let b = a.clone();
    let b = &a;
    println!("{:?} {:?}", a, b);
}

fn main() {
    demo3();
}
// struct MyBox<T>(T);

// impl<T> MyBox<T> {
//     fn new(value: T) -> MyBox<T> {
//         Self(value)
//     }
// }

// impl<T> Deref for MyBox<T> {
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
// fn main() {
//     let x = 5;
//     let y = MyBox::new(x);
//     println!("{}", x == *y);
// println!("hello world");
// let str = "Pascal Suite";
// greet(str);
// let s = "Hello World";
// let s1 = s;
// println!("{}", s1);
// println!("{}", s);
// let s = String::from("Hello World");
// let s1 = &s;
// println!("{}", s1);
// println!("{}", s[1]);
// let vec = vec![1, 2, 3];
// let vec1 = &vec;
// println!("{:?}", vec1);
// println!("{:?}", vec);
// let mut map = HashMap::new();
// map.insert("hello", "world");
// let map1 = &map;
// println!("{:?}", map1);
// println!("{:?}", map);
// let tup = (500, "hello", String::from("World"));
// let tup1 = tup;
// println!("{:?}", tup);
// println!("{:?}", tup1);
// let x = [String::from("Hello World"), String::from("111")];
// let x1 = x;
// println!("{:?}", x1);
// println!("{:?}", x);
// let x = [1, 2];
// let x1 = x;
// println!("{:?}", x1);
// println!("{:?}", x);
// let a: i32 = 1;
// let b = a;
// println!("{:?}", a);
// println!("{:?}", &b);
// let x = 5;
// let y = &x;
// assert_eq!(x, *y);
// println!("{:p}", &x);
// println!("{:p}", y);
// let x = 5;
// let y = Box::new(x);
// assert_eq!(5, x);
// assert_eq!(5, *y);
// println!("{:p}", y);
// }

// fn greet(str: String) {
//     println!("Hello, {}", str);
// }
