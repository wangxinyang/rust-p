use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (sx, rx) = mpsc::channel();

    let sx1 = mpsc::Sender::clone(&sx);

    thread::spawn(move || {
        let val = vec!["hi", "hello", "fine"];
        for message in val {
            sx.send(message).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    thread::spawn(move || {
        let val = vec!["a", "b", "c"];
        for message in val {
            sx1.send(message).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for message in rx {
        println!("Got: {}", message);
    }
}
