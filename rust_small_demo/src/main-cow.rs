use std::borrow::Cow;

fn main() {
    let strs = vec!["row_rust".to_string(), "rust".to_string()];
    let p = "row_";
    // let fixed = insert_prefix_clone(&strs, p);
    let fixed = insert_prefix_cow(&strs, p);
    println!("{:?}", fixed);

    let s0 = &strs[0]; // 第一个元素已经有指定前缀名了
    let f0 = &*fixed[0];

    println!("source addr: {:?}", s0 as *const String); // 0x7fd27ff05cf0
    println!("clone addr: {:?}", f0 as *const String); //  0x7fd27ff05cf0地址相同
}

fn insert_prefix_cow<'a>(strs: &'a Vec<String>, prefix: &str) -> Vec<Cow<'a, String>> {
    strs.into_iter()
        .filter_map(|s| match s.starts_with(prefix) {
            true => Some(Cow::Borrowed(s)),
            false => Some(Cow::Owned(
                String::with_capacity(prefix.len() + s.len()) + prefix + &s,
            )),
        })
        .collect::<Vec<Cow<'a, String>>>()
}

fn insert_prefix_clone(strs: &Vec<String>, prefix: &str) -> Vec<String> {
    strs.into_iter()
        .filter_map(|s| match s.starts_with(prefix) {
            true => Some(s.clone()),
            false => Some(String::with_capacity(prefix.len() + s.len()) + prefix + &s),
        })
        .collect::<Vec<String>>()
}
