fn main() {
    // let x = 4;
    // let equal_to_x = |y: i32| y == x;
    // assert!(equal_to_x(x));
    let x = vec![1, 2, 3];
    let equal_to_x = move |y: Vec<i32>| y == x;
    println!("{:?}", x);
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}
