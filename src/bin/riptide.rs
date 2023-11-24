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
    let config = get_configuration();
    let folder_path = config.path.path;
    let editor = config.editor.editor;
    let mut remote_address = "".to_string();
    let mut remote_execution = false;
    let mut cat_script = false;
    let mut edit_script = false;
    let args: Vec<String> = env::args().collect();
    let mut script_name: &String;
    let mut script_arguments: Vec<String>;
    let concated_script_arguments: String;

    if args.len() < 2 {
        println!("riptide takes application name as an argument");
        return;
    }

    script_name = &args[1];
    script_arguments = args.clone();
    script_arguments.drain(0..2);
    if &args[1] == "-c" || &args[1] == "--cat" {
        cat_script = true;
        script_name = &args[2];
    }
    if &args[1] == "-e" || &args[1] == "--edit" {
        edit_script = true;
        script_name = &args[2];
    }

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
    concated_script_arguments = script_arguments
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    match cat_script {
        true => {
            let mut command = Command::new("sh");
            command
                .arg("-c")
                .arg(format!("cat {}{}", folder_path, script_name));
            execute_command(command);
        }
        false => match edit_script {
            true => {
                let mut command = Command::new("sh");
                command
                    .arg("-c")
                    .arg(format!("{} {}{}", editor, folder_path, script_name));
                execute_command(command);
            }
            false => match remote_execution {
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
            },
        },
    }
}
