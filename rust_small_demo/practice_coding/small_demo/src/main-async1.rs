use futures::executor;

async fn hello() {
    println!("Hello");
}

fn main() {
    let f = hello();
    executor::block_on(f);
    println!("Hello World");
}
