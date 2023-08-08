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
    let script_name = &args[1];
    let mut script_arguments = args.clone();
    script_arguments.drain(0..2);

    let concated_script_arguments = script_arguments
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    let mut command = Command::new("sh");

    command.arg("-c").arg(format!(
        "{}{} {}",
        folder_path, script_name, concated_script_arguments
    ));

    for script_argument in script_arguments {
        command.arg(script_argument);
    }

    command
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit());

    let mut child = command.spawn().expect("Failed to execute command.");

    child.wait().expect("Failed to wait for command execution.");
}
