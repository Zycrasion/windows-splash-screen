use std::{
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

use iced::{
    Element,
    Length::Fill,
    Size, Task,
    widget::{image::Handle, *},
};

const IMAGE_BYTES: &[u8] = include_bytes!("image.jpg");

struct WaitFuture {
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

#[derive(Clone, Copy, Debug)]
enum Message {
    Die,
}

pub struct Shell {
    image: Handle,
}

impl Shell {
    pub fn start() {
        iced::application(Self::title, Self::update, Self::view)
            .window(iced::window::Settings {
                decorations: false,
                ..Default::default()
            })
            .centered()
            .window_size(Size::new(300., 300.))
            .run_with(|| {
                (
                    Shell::new(),
                    Task::future(WaitFuture::new(Duration::from_secs(2))),
                )
            })
            .unwrap();
    }

    fn new() -> Self {
        Self {
            image: Handle::from_bytes(IMAGE_BYTES),
        }
    }

    fn title(&self) -> String {
        String::from("Shell")
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Die => {
                return iced::window::get_latest().then(|id| iced::window::close(id.unwrap()));
            }
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        container(image(self.image.clone()).height(Fill).width(Fill)).into()
    }
}
