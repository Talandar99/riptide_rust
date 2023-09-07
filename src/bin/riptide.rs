use lib::configuration::*;
use std::env;
use std::process::Command;

fn execute_command(mut command: Command) {
    command
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit());
    let mut child = command.spawn().expect("Failed to execute command.");
    child.wait().expect("Failed to wait for command execution.");
}

fn main() {
    let config = get_confuguration();
    let folder_path = config.path.path;
    let mut remote_address = "".to_string();
    let mut remote_execution = false;

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("riptide takes application name as an argument");
        return;
    }
    let script_name = &args[1];
    let mut script_arguments = args.clone();
    script_arguments.drain(0..2);

    for (index, _arg) in args.iter().enumerate() {
        if _arg == "-r" || _arg == "--remote" {
            remote_execution = true;
            println!("-------------------------------");
            println!("Remote Execution of {}", script_name);
            println!("at {}", script_arguments[index - 1]);
            println!("-------------------------------");
            remote_address = script_arguments[index - 1].clone();

            script_arguments.drain((index - 2)..);
            break;
        }
    }
    let concated_script_arguments = script_arguments
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    match remote_execution {
        true => {
            let mut command = Command::new("sh");
            command.arg("-c").arg(format!(
                "scp {}{} {}:~/{} ",
                folder_path, script_name, remote_address, script_name
            ));
            execute_command(command);
            let mut command = Command::new("sh");
            command.arg("-c").arg(format!(
                "ssh {} ./{} {}",
                remote_address, script_name, concated_script_arguments
            ));
            for script_argument in script_arguments {
                command.arg(script_argument);
            }
            execute_command(command);
            let mut command = Command::new("sh");
            command
                .arg("-c")
                .arg(format!("ssh {} rm {} ", remote_address, script_name));
            execute_command(command);
        }
        false => {
            let mut command = Command::new("sh");
            command.arg("-c").arg(format!(
                "{}{} {}",
                folder_path, script_name, concated_script_arguments
            ));
            for script_argument in script_arguments {
                command.arg(script_argument);
            }
            execute_command(command);
        }
    }
}
