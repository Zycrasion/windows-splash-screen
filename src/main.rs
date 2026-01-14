#![windows_subsystem = "windows"]

use std::thread;
use std::time::Duration;
use std::{env, process::Command};

use utils::ui::message_box::{error_box, information_box, yes_no_box};
use utils::*;
use windows::Win32::Foundation::{ERROR_FILE_NOT_FOUND, ERROR_PATH_NOT_FOUND};
use windows::Win32::UI::Shell::{ShellExecuteA, ShellExecuteW};
use windows::Win32::UI::WindowsAndMessaging::SHOW_WINDOW_CMD;
use windows::core::PCSTR;
use windows_registry::LOCAL_MACHINE;

fn elevate(exe_path: &str, args: &str) {
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

const WINLOGON: &str = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Winlogon";
fn read_shell_path(path: &str, val: &str) -> String {
    LOCAL_MACHINE
        .open(path)
        .unwrap_or_else(|v| {
            error!("{v}");
            error_box("Error", format!("{v}"));
            panic!();
        })
        .get_string(val)
        .unwrap_or_else(|v| {
            error!("{v}");
            error_box("Error", "{v}");
            panic!();
        })
}

fn set_shell_path(path: &str, val: &str, to: &str) {
    LOCAL_MACHINE
        .create(path)
        .unwrap_or_else(|v| {
            error!("{v}");
            error_box("Error", format!("{v}"));
            panic!();
        })
        .set_string(val, to)
        .unwrap_or_else(|v| {
            error!("{v}");
            error_box("Error", "{v}");
            panic!();
        })
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let exe_path_path = env::current_exe().unwrap_or_else(|v| {
        error!("{v}");
        error_box("Error", format!("{v}"));
        panic!();
    });

    let exe_path = exe_path_path.to_str().unwrap();

    if args.contains(&"--restore".to_string()) {
        restore();
        information_box("Shell", "Restored to default settings.");
        return;
    } else if args.contains(&"--install".to_string()) {
        install(exe_path);
        information_box("Shell", "Please sign in and out to see changes in effect");
        return;
    } else if args.contains(&"--launch-shell".to_string()) {
        restore();
        thread::sleep(Duration::from_secs_f64(0.2));
        Command::new("C:/Windows/explorer.exe")
            .spawn()
            .unwrap_or_else(|v| {
                error!("{v}");
                error_box("Error", format!("{v}"));
                panic!();
            });
        thread::sleep(Duration::from_secs_f64(0.2));
        install(exe_path);
        return;
    }

    if read_shell_path(WINLOGON, "Shell") != get_shell_command(exe_path) {
        if !yes_no_box("Installer", "Install shell?") {
            return;
        }
        elevate(exe_path, "--install");
    } else {
        shell(exe_path);
    }

    // if yes_no_box("Restore", "Restore?") {
    //     elevate(exe_path, "--restore");
    //     return;
    // }
}

fn get_shell_command(exe_path: &str) -> String {
    format!("{exe_path}")
}

fn restore() {
    set_shell_path(WINLOGON, "Shell", "explorer.exe");
}

fn install(exe_path: &str) {
    let modified = get_shell_command(exe_path);
    set_shell_path(WINLOGON, "Shell", &modified);
}

fn shell(exe_path: &str) {
    custom_shell::shell::Shell::start();

    // while !yes_no_box("Shell", "Start shell?") {}
    elevate(exe_path, "--launch-shell");
}
