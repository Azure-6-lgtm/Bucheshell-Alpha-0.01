use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use colored::*;
use rustyline::DefaultEditor;

fn get_display_dir() -> String {
    let dir = env::current_dir().unwrap();
    let home = env::var("HOME").unwrap();

    let path = dir.display().to_string();

    if path.starts_with(&home) {
        path.replacen(&home, "~", 1)
    } else {
        path
    }
}

fn main() {
    let mut rl = DefaultEditor::new().unwrap();
    println!("Buche Shell 0.01 Testing enviorment");
    println!("Made with love ");
    println!("------------------------------------");
    ctrlc::set_handler(|| {}).unwrap();
    loop {
        let cpath = get_display_dir();
        let prompt = format!("[{}]{}",cpath.cyan(),">>".cyan());
        //print!("[{}] >>",get_display_dir());
        let input_command = match rl.readline(&prompt) {
    Ok(input) => {
        rl.add_history_entry(input.as_str());
        input
    }
    Err(_) => break,
};

let input_command = input_command.trim();

        io::stdout().flush().unwrap();

        if input_command.is_empty() {
            continue;
        }
        let mut parts = input_command.split_whitespace();
        let cmd = parts.next().unwrap();
        let cmdargs: Vec<&str> = parts.collect();

        let path = format!("/bin/{}", cmd);
        let usrpath = format!("/usr/bin/{}", cmd);
        let termuxpath = format!("/data/data/com.termux/files/usr/bin/{}", cmd);

        if Path::new(&path).exists() && &path != "/bin/ping" {
            Command::new(&path)
                .args(&cmdargs)
                .status()
                .expect("Bucheshell failed to run the binary");
        } else if Path::new(&usrpath).exists() && &usrpath != "usr/bin/ping" {
            Command::new(&usrpath)
                .args(&cmdargs)
                .status()
                .expect("Bucheshell failes to run the bimary");
        } else if Path::new(&termuxpath).exists() && &termuxpath != "/data/data/com.termux/files/usr/bin/ping" {
            Command::new(&termuxpath)
                .args(&cmdargs)
                .status()
                .expect("Bucheshell couldnt run binary");
        } else {
            match cmd {
                "exit" => break,
                "bshabout" => aboutbsh(),
                "cd" => {
                    builtin_cd(&cmdargs);
                }
                "ver" => {
                    bshversion();
                }
                "help" => {
                    help();
                }
                "ping" => {
                    Command::new("ping")
                        .args(&cmdargs)
                        .status()
                        .expect("Bucheshell couldnt run binary");
                }
                _ => println!("Could not find command in bsh library: {}", cmd),
            }
        };
    }
}

fn aboutbsh() {
    println!("Bucheshell is a shell developed for termux use cases");
}
fn builtin_cd(args: &[&str]) {
    use std::env;
    use std::path::PathBuf;

    let home = env::var("HOME").unwrap_or(String::from("/"));

    let target = if args.is_empty() {
        PathBuf::from(&home)
    } else if args[0].starts_with("~") {
        PathBuf::from(args[0].replacen("~", &home, 1))
    } else {
        PathBuf::from(args[0])
    };

    if let Err(e) = env::set_current_dir(&target) {
        println!("cd: {}", e);
    }
}
fn bshversion() {
    println!("Buche shell version 0.01 Alpha release");
}
fn help() {
    println!("WIP!");
}
