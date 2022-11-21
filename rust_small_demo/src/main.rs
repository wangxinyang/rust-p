fn main() {
    let str1 = "tosei";
    // because impl ToOwned for str
    // Owned = String
    let str2 = str1.to_owned();
    print_type_of(&str2);
    println!("str2 is: {:?}", str2);

    // &str -> String
    let str = str1.to_string();
    print_type_of(&str);
    println!("str is: {:?}", str);

    // &str -> bytes slice
    let str1_bytes = str1.as_bytes();
    print_type_of(&str1_bytes);
    println!("str1_bytes is: {:?}", str1_bytes);

    // bytes slice -> bytes array
    // let length = str1_bytes.len();
    // attempt to use a non-constant value in a constant non-constant value
    // let str1_num: [u8; length] = str1_bytes.try_into().unwrap();
    // array length must be the same as the slice length
    let str1_num: [u8; 5] = str1_bytes.try_into().unwrap();
    print_type_of(&str1_num);
    println!("str1_num is: {:?}", str1_num);

    // bytes slice -> Vec
    // because impl ToOwned for [T], Owned = Vec<T>
    let vec_from_slice = str1_bytes.to_owned();
    print_type_of(&vec_from_slice);
    println!("vec_from_slice is {:?}", vec_from_slice);

    // bytes slice -> Vec
    let str1_vec = str1_bytes.to_vec();
    print_type_of(&str1_vec);
    println!("str1_vec is {:?}", str1_vec);

    // Vec -> String
    let str_from_vec = String::from_utf8(str1_vec).unwrap();
    print_type_of(&str_from_vec);
    println!("str_from_vec is {:?}", str_from_vec);

    // String -> Vec
    let vec_from_string = str_from_vec.into_bytes();
    print_type_of(&vec_from_string);
    println!("str_from_vec is {:?}", vec_from_string);
}

fn print_type_of<T>(_: &T) {
    println!("type is = {}", std::any::type_name::<T>())
}
