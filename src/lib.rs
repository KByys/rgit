use std::fs::metadata;
use std::process::Command;

use iofs::env::{current_dir_path, current_program_path};
use iofs::fs::pbuilder::PathBuilder;
use iofs::prelude::{Console, DFiles, DirectoryInfo};
#[cfg(windows)]
static PROGRAM: &str = "powershell";

#[cfg(unix)]
static PROGRAM: &str = "shell";

pub fn run(args: Vec<String>) {
    if args.len() > 1 {
        let path = current_program_path().expect("Unknown Error!");
        let path = PathBuilder::from(path);
        let path = format!("{}/data", path.parent());
        match args[1].to_ascii_lowercase().as_str() {
            "login" => {
                let Some(username) = args.get(2) else { 
                    Console::print("rgit login ", None, None); 
                    Console::println("<username>", Some(196), None);
                    return; 
                };
                std::fs::write(path, username.as_bytes()).expect("Write Login Messages Error");
                println!("Login successful")
            }
            "push" => {
                // push . <des> <master>

                if args.len() >= 5 {
                    let mut command = Command::new(PROGRAM);
                    let _ = command
                        .args(["git", "add", args[2].as_str()])
                        .status()
                        .unwrap();
                    let _ = command
                        .args(["git", "commit", "-m", args[3].as_str()])
                        .status()
                        .unwrap();
                    let _ = command
                        .args(["git", "push", "-u", "origin", args[4].as_str()])
                        .status()
                        .unwrap();
                }
            }
            // Clone
            _ => {
                let http = "https://github.com/";
                let username =
                    std::fs::read_to_string(path).expect("Read Login Messages Error");
                let address = format!("git clone {http}{}/{}.git", username, args[1]);
                let _ = Command::new(PROGRAM).arg(address).status().unwrap();
            }
        }
    }
}
