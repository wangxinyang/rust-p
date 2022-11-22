use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (sender, receiver) = mpsc::channel();

    thread::spawn(move || {
        let val = vec!["hello", "hi", "how are you", "fine 3q"];
        for message in val {
            sender.send(message).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for message in receiver {
        println!("received message: {}", message);
    }
}
