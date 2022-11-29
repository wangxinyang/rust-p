use futures::executor::block_on;

fn main() {
    // 本身不会执行 使用执行器
    // do_something();
    let future = do_something();
    block_on(future);
}

async fn do_something() {
    println!("Hello, world!");
}
