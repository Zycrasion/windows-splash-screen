use std::{
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

use crate::shell::Message;

pub struct WaitFuture<T> {
    start: Instant,
    wait_for: Duration,
    return_val: T,
}

impl<T> WaitFuture<T> {
    pub fn new(wait_for: Duration, return_val: T) -> Self {
        WaitFuture {
            start: Instant::now(),
            wait_for,
            return_val,
        }
    }
}

impl<T: Copy> Future for WaitFuture<T> {
    type Output = T;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if self.start.elapsed() >= self.wait_for {
            return std::task::Poll::Ready(self.return_val);
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
