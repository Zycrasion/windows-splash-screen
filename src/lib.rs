use std::fmt::Display;

use utils::{error, ui::message_box::error_box};
use windows::{
    Win32::UI::{Shell::ShellExecuteA, WindowsAndMessaging::SHOW_WINDOW_CMD},
    core::PCSTR,
};
use windows_registry::CURRENT_USER;

pub mod futures;
pub mod shell;

pub const WINLOGON: &str = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Winlogon";

#[macro_export]
macro_rules! handle_error {
    () => {
        |v| error(v, line!(), file!())
    };
}

pub fn error<T: Display, V>(error: T, line: u32, file: &str) -> V {
    error!("{file}:{line} {}", error);
    error_box("Error", format!("{file}:{line} {error}"));
    std::process::exit(0);
}

pub fn read_usr_path(path: &str, val: &str) -> Option<String> {
    CURRENT_USER
        .open(path)
        .unwrap_or_else(handle_error!())
        .get_string(val)
        .ok()
}

pub fn set_usr_path(path: &str, val: &str, to: &str) {
    CURRENT_USER
        .create(path)
        .unwrap_or_else(handle_error!())
        .set_string(val, to)
        .unwrap_or_else(handle_error!())
}

pub fn elevate(exe_path: &str, args: &str) {
    let terminated = format!("{exe_path}\0");
    let exe_terminated = PCSTR::from_raw(terminated.as_ptr());

    unsafe {
        ShellExecuteA(
            None,
            PCSTR::from_raw("runas\0".as_ptr()),
            exe_terminated,
            PCSTR::from_raw(format!("{args}\0").as_ptr()),
            PCSTR::null(),
            SHOW_WINDOW_CMD::default(),
        )
    };
}
