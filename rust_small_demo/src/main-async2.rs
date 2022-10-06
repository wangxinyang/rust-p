use futures::executor;

async fn learn_song() {
    println!("learn_song");
}

async fn sing_song() {
    println!("sing_song");
}

async fn dance() {
    println!(" dance");
}

async fn learn_sing_song() {
    learn_song().await;
    sing_song().await;
}

async fn async_main() {
    let f1 = learn_sing_song();
    let f2 = dance();
    futures::join!(f1, f2);
}

fn main() {
    executor::block_on(async_main());
    println!("Hello World");
}
