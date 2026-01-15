#![windows_subsystem = "windows"]

use std::{env, process::Command};

use custom_shell::{WINLOGON, error, handle_error, read_usr_path, set_usr_path, shell::Shell};
use utils::ui::message_box::information_box;

fn main() {
    let exe_path_path = env::current_exe().unwrap_or_else(handle_error!());

    let exe_path = exe_path_path.to_str().unwrap();

    let current_shell = read_usr_path(WINLOGON, "Shell");

    if current_shell.is_some() && current_shell.unwrap() == exe_path {
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
