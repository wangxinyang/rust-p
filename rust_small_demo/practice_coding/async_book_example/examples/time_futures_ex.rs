mod executor;
mod time_future;

use std::time::Duration;

pub use executor::*;
pub use time_future::*;

fn main() {
    let (executor, spawner) = new_executor_and_spawner();

    spawner.spawn(async {
        println!("howdy");
        // create time sleep
        TimeFurture::new(Duration::new(2, 0)).await;
        println!("done!");
    });

    drop(spawner);

    executor.run();
}
