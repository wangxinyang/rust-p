use std::{thread, time::Duration};

use futures::executor::block_on;

async fn hello_world() {
    hello_cat().await;
    thread::sleep(Duration::from_secs(5));
    println!("Hello, world!");
}

async fn hello_cat() {
    println!("Hello, cat!");
}

fn main() {
    let a = 10;
    let b = 20;
    let c = a;

    let future = hello_world();
    block_on(future);
}
