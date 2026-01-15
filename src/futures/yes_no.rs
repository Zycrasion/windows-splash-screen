use std::{
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

use utils::ui::message_box::{win, yes_no_box};

use crate::shell::Message;

pub struct YesNoBoxFuture {
    title: String,
    caption: String,
    id: u32,
}

impl YesNoBoxFuture {
    pub fn new<S: AsRef<str>, S2: AsRef<str>>(id: u32, title: S, caption: S2) -> Self {
        YesNoBoxFuture {
            id,
            title: title.as_ref().to_string(),
            caption: caption.as_ref().to_string(),
        }
    }
}

impl Future for YesNoBoxFuture {
    type Output = Message;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        std::task::Poll::Ready(Message::YesNo(
            self.id,
            win::win_message_box(
                &self.title,
                &self.caption,
                &win::MessageBoxType::new()
                    .icon_question()
                    .yes_no()
                    .system_modal()
                    .top_most(),
            ) == win::WinMessageBoxResult::Yes,
        ))
    }
}
