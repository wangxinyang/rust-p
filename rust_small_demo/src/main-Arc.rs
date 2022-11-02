use std::sync::{Arc, Barrier};

fn main() {
    let mut handles = Vec::with_capacity(6);
    let barrier = Arc::new(Barrier::new(6));

    for _ in 0..6 {
        let c = barrier.clone();
        handles.push(std::thread::spawn(move || {
            println!("before wait");
            c.wait();
            println!("after wait");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
