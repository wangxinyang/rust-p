use std::{
    mem,
    time::{SystemTime, UNIX_EPOCH},
};

fn main() {
    let sys_time = SystemTime::now();
    println!("sys_time is:{sys_time:?}");
    let new_sys_time = SystemTime::now();
    let difference = new_sys_time
        .duration_since(UNIX_EPOCH)
        .expect("Clock may have gone backwards");
    // let difference = new_sys_time
    //     .duration_since(sys_time)
    //     .expect("Clock may have gone backwards");
    println!("{difference:?}");

    let a = String::from("Hello");
    println!("{}", mem::size_of_val(&a));
}
