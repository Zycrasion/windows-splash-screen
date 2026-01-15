use std::time::Duration;

use iced::{
    Element,
    Length::Fill,
    Size, Task,
    widget::{image::Handle, *},
};

use crate::futures::time_delay::WaitFuture;

const IMAGE_BYTES: &[u8] = include_bytes!("image.jpg");

#[derive(Clone, Copy, Debug)]
pub enum Message {
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
