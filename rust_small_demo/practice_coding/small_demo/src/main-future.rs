use std::{thread, time::Duration};

#[tokio::main]
async fn main() {
    let hello = Hello::Started;
    let result = hello.await;
    println!("Result: {:?}", result);
}

enum Hello {
    Started,
    Working,
    _Done,
}

impl std::future::Future for Hello {
    type Output = i32;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let wake = cx.waker().clone();
        let h = self.get_mut();
        match h {
            Hello::Started => {
                *h = Hello::Working;
                thread::sleep(Duration::from_secs(1));
                wake.wake();
                std::task::Poll::Pending
            }
            Hello::Working => std::task::Poll::Ready(110),
            Hello::_Done => {
                panic!("Not here");
            }
        }
    }
}
