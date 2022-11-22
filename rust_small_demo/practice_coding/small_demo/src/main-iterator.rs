fn main() {
    let number = vec![1, 2, 3, 4, 5];

    // for num in number {
    //     println!("{:p}", &&num);
    // }

    let find_number = number.iter().find(|x| x == &&3).unwrap();
    println!("{:?}", find_number);

    // let filter_number: Vec<_> = number.iter().filter(|x| x > &&3).collect();
    // println!("{:?}", filter_number);

    // let filter_number: Vec<_> = number.iter().filter(|x| **x > 3).collect();
    // println!("{:?}", filter_number);

    let final_number: Vec<_> = number
        .iter()
        .map(|x| x * 10)
        .filter(|x| x > &10)
        .take(2)
        .collect();

    println!("{:?}", final_number);
}
