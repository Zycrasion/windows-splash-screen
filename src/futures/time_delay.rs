use std::{
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

use crate::shell::Message;

pub struct WaitFuture {
    start: Instant,
    wait_for: Duration,
}

impl WaitFuture {
    pub fn new(wait_for: Duration) -> Self {
        WaitFuture {
            start: Instant::now(),
            wait_for,
        }
    }
}

impl Future for WaitFuture {
    type Output = Message;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if self.start.elapsed() >= self.wait_for {
            return std::task::Poll::Ready(Message::Die);
        }

        let waker = Arc::new(cx.waker().clone());
        let wait_for = self.wait_for.clone();

        thread::spawn(move || {
            thread::sleep(wait_for);
            waker.wake_by_ref();
        });

        std::task::Poll::Pending
    }
}
