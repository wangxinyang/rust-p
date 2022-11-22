use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("number {} in spawn threa", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();
    for i in 1..5 {
        println!("number {} in main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
    // handle.join().unwrap();
    println!("hello world");
}
