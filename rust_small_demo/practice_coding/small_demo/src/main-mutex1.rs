use std::sync::Mutex;

fn main() {
    let mu = Mutex::new(5);

    {
        let mut data = mu.lock().unwrap();
        *data = 6;
    }

    println!("{:?}", mu);
}
