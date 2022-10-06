fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut larger = list[0];
    for &item in list.iter() {
        if item > larger {
            larger = item;
        }
    }
    larger
}

fn main() {
    let number_list = vec![0, 1, 2, 3, 4, 5];
    let max_number = largest(&number_list);
    println!("{}", max_number);
}
