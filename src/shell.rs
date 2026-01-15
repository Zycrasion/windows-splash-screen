use std::time::Duration;

use iced::{
    Element,
    Length::Fill,
    Size, Task,
    widget::{image::Handle, *},
};
use windows::Win32::System::Shutdown::{EWX_LOGOFF, ExitWindowsEx, SHUTDOWN_REASON};

use crate::{
    error,
    futures::{time_delay::WaitFuture, yes_no::YesNoBoxFuture},
    handle_error,
};

const IMAGE_BYTES: &[u8] = include_bytes!("image.jpg");

const DO_YOU_LOVE_ME: u32 = 0;
const ARE_YOU_SURE: u32 = 1;
const LAST_CHANCE: u32 = 2;

#[derive(Clone, Copy, Debug)]
pub enum Message {
    Die,
    YesNo(u32, bool),
    WaitDone,
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
                    Task::future(WaitFuture::new(
                        Duration::from_secs_f32(1.5),
                        Message::WaitDone,
                    )),
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
            Message::WaitDone => {
                return Task::future(YesNoBoxFuture::new(
                    DO_YOU_LOVE_ME,
                    "Teto",
                    "Do you love me?",
                ));
            }
            Message::YesNo(id, result) => match id {
                DO_YOU_LOVE_ME => {
                    if result {
                        return Task::done(Message::Die);
                    } else {
                        return Task::future(YesNoBoxFuture::new(
                            ARE_YOU_SURE,
                            "Teto",
                            "Are you sure?",
                        ));
                    }
                }
                ARE_YOU_SURE => {
                    if result {
                        return Task::done(Message::Die);
                    } else {
                        return Task::future(YesNoBoxFuture::new(
                            LAST_CHANCE,
                            "Teto",
                            "Last chance. Do you love me?",
                        ));
                    }
                }
                LAST_CHANCE => {
                    if result {
                        return Task::done(Message::Die);
                    } else {
                        unsafe { ExitWindowsEx(EWX_LOGOFF, SHUTDOWN_REASON::default()) }
                            .unwrap_or_else(handle_error!());
                    }
                }
                _ => {
                    return Task::done(Message::Die);
                }
            },
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
