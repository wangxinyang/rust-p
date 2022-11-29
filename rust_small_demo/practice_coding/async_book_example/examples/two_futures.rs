use std::time::Duration;

use futures::executor::block_on;

fn main() {
    let future = hello_world();
    // block_on会阻塞线程
    block_on(future);
    println!("waiting for future");
}

async fn hello_world() {
    // 等待其它future执行完成,因为没有任何人去执行这个future,需要使用await
    // hello_cat();
    hello_cat().await;
    println!("hello, world!");
    std::thread::sleep(Duration::from_secs(5));
}

async fn hello_cat() {
    println!("hello, cat!");
}
