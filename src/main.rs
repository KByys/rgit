use std::process::Command;
use std::env;
use iofs::env::current_program_path;
use iofs::fs::pbuilder::PathBuilder;
fn main() {
    let args = env::args().collect::<Vec<String>>();
    let path = current_program_path().unwrap();
    let f = PathBuilder::from(path);
    let path = format!("{}/data", f.parent());
    if args.len() > 2 {
        if args[1].eq_ignore_ascii_case("login") {
            std::fs::write(path, args[2].as_bytes()).expect("Login Error");   
            println!("登录成功")
        }
    } else if args.len() == 2 {
        let http = "https://github.com/";
        let username = std::fs::read_to_string(path).expect("Read Login Message Error");
        let address = format!("git clone {http}{}/{}.git", username, args[1]);
        let _ = Command::new("powershell").arg(address).status().unwrap();
    } else {
        println!("Nothing happened")
    }
}

