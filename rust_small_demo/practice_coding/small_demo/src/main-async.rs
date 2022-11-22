use std::{
    sync::{
        mpsc::{sync_channel, Receiver, SyncSender},
        Arc, Mutex,
    },
    task::{Context, Poll, Waker},
    thread::{self, JoinHandle},
    time::Duration,
};

use futures::{
    future::BoxFuture,
    task::{waker_ref, ArcWake},
    Future, FutureExt,
};

fn main() {
    // println!("thread before");
    // let mut t: Vec<JoinHandle<()>> = Vec::new();
    // let t1 = thread::spawn(|| println!("{:?} thread1 run", thread::current().id()));
    // t.push(t1);
    // let t2 = thread::spawn(|| println!("{:?} thread2 run", thread::current().id()));
    // t.push(t2);
    // println!("thread after");
    // for item in t {
    //     item.join().unwrap();
    // }
    let (executor, spawner) = new_executor_and_spawn();
    spawner.spawn(async {
        println!("howdy!");
        // Wait for our timer future to complete after two seconds.
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("done!");
    });
    drop(spawner);
    executor.run();
}

fn new_executor_and_spawn() -> (MyExecutor, MySpawner) {
    let bound = 10_000;
    let (task_sender, task_recv) = sync_channel(bound);
    (MyExecutor { task_recv }, MySpawner { task_sender })
}

struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

struct SharedState {
    complete: bool,
    waker: Option<Waker>,
}

impl TimerFuture {
    fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            complete: false,
            waker: None,
        }));

        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            shared_state.complete = true;
            if let Some(wake) = shared_state.waker.take() {
                wake.wake();
            }
        });

        TimerFuture { shared_state }
    }
}

impl Future for TimerFuture {
    type Output = ();

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.complete {
            Poll::Ready(())
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

struct MyExecutor {
    task_recv: Receiver<Arc<Task>>,
}

struct MySpawner {
    task_sender: SyncSender<Arc<Task>>,
}

struct Task {
    future: Mutex<Option<BoxFuture<'static, ()>>>,
    task_sender: SyncSender<Arc<Task>>,
}

impl MySpawner {
    fn spawn(&self, future: impl Future<Output = ()> + Send + 'static) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).unwrap();
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.task_sender.send(arc_self.clone()).unwrap();
    }
}

impl MyExecutor {
    fn run(&self) {
        while let Ok(task) = self.task_recv.recv() {
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                let waker = waker_ref(&task);
                let context = &mut Context::from_waker(&waker);
                if future.as_mut().poll(context).is_pending() {
                    // We're not done processing the future, so put it
                    // back in its task to be run again in the future.
                    *future_slot = Some(future);
                }
            }
        }
    }
}
