use lib::configuration::*;
use std::env;
use std::process::Command;

fn main() {
    let config = get_confuguration();
    let folder_path = config.path.path;
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("riptide takes application name as an argument");
        return;
    }
    let application_name = &args[1];
    for _i in 2..args.len() {
        print!("{}", args[2])
    }

    let mut command = Command::new("sh");

    command
        .arg("-c")
        .arg(format!("{}{}", folder_path, application_name));

    command
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit());

    let mut child = command.spawn().expect("Failed to execute command.");

    child.wait().expect("Failed to wait for command execution.");
}
