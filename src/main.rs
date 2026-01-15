#![windows_subsystem = "windows"]

use std::{
    env, fs::File, io::Read, os::windows::fs::OpenOptionsExt, process::Command, time::Duration,
};

use custom_shell::{WINLOGON, error, handle_error, read_usr_path, set_usr_path, shell::Shell};
use device_query::{DeviceEvents, DeviceEventsHandler};
use iced::Length::Fill;
use utils::{info, ui::message_box::information_box};
use windows::Win32::System::Shutdown::{EWX_LOGOFF, ExitWindowsEx, SHUTDOWN_REASON};

fn main() {
    let exe_path_path = env::current_exe().unwrap_or_else(handle_error!());

    let exe_path = exe_path_path.to_str().unwrap();

    let current_shell = read_usr_path(WINLOGON, "Shell");

    if current_shell.is_some() && current_shell.unwrap() == exe_path {
        let state =
            DeviceEventsHandler::new(Duration::from_millis(10)).unwrap_or_else(handle_error!(no));

        let guard = state.on_key_down(|key| match key {
            device_query::Keycode::Delete
            | device_query::Keycode::RControl
            | device_query::Keycode::LControl => {
                unsafe { ExitWindowsEx(EWX_LOGOFF, SHUTDOWN_REASON::default()) }
                    .unwrap_or_else(handle_error!())
            }
            _ => {}
        });

        Shell::start();
        Command::new("C:/Windows/explorer.exe")
            .spawn()
            .unwrap_or_else(handle_error!());
    } else {
        install(exe_path);
        information_box(
            "Shell",
            "Installation Successful; Log in and out to see changes",
        );
    }
}

fn install(exe_path: &str) {
    set_usr_path(WINLOGON, "Shell", exe_path);
}

fn uninstall() {
    set_usr_path(WINLOGON, "Shell", "explorer.exe");
}
