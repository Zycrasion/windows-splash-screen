use std::{io::Cursor, sync::Arc, thread};

use rodio::{OutputStream, Sink, mixer::Mixer};

pub const DO_YOU_LOVE_ME: &[u8] = include_bytes!("../do_u_love_me.wav");
pub const DO_YOU_NEED_ME: &[u8] = include_bytes!("../do_u_need_me.wav");
pub const THANK_YOU: &[u8] = include_bytes!("../thank_you_kitty.wav");
pub const YOU_DID_NOT_JUST_SAY_THAT: &[u8] = include_bytes!("../you_did_not_just_say_that.wav");

pub fn play_file(mixer: &Arc<OutputStream>, bytes: &[u8]) {
    let sink = rodio::play(mixer.mixer(), Cursor::new(bytes.to_vec())).unwrap();
    sink.detach();
}

pub fn play_file_sync(mixer: &Arc<OutputStream>, bytes: &[u8]) {
    let sink = rodio::play(mixer.mixer(), Cursor::new(bytes.to_vec())).unwrap();
    sink.sleep_until_end();
}
