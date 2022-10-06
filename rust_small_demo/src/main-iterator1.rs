fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 7];
    let v_iter = v.iter();
    let total: i32 = v_iter.sum();
    println!("total is = {}", total);

    let v1 = vec![1, 2, 3, 4, 5, 6, 7];
    let v1_iter = v1.into_iter();
    let v2: Vec<_> = v1_iter.filter(|x| *x > 5).collect();
    println!("v2 is = {:?}", v2);

    let v3 = vec![1, 2, 3, 4, 5, 6, 7];
    let v3_iter = v3.iter();
    let v4: Vec<_> = v3_iter.filter(|x| **x > 5).collect();
    println!("v4 is = {:?}", v4);
}
