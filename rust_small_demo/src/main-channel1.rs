use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        sender.send(val).unwrap();
        thread::sleep(Duration::from_millis(2));
    });

    let received = receiver.recv().unwrap();
    println!("Got: {}", received);
}
