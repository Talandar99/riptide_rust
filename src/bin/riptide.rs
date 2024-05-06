use lib::configuration::*;
use std::process::Command;
use std::{env, io};

enum Flag {
    Cat,
    None,
    RemoteSsh,
    Edit,
}

fn execute_command(formated_command: String) {
    let mut command = Command::new("sh");
    command.arg("-c").arg(formated_command);
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
    let args: Vec<String> = env::args().collect();
    let mut script_arguments: Vec<String>;
    let concated_script_arguments: String;
    let mut flag: Flag = Flag::None;

    if args.len() < 2 {
        println!("riptide takes application name as an argument");
        return;
    }

    script_arguments = args.clone();
    script_arguments.drain(0..1);

    for (index, _arg) in args.iter().enumerate() {
        if _arg.to_lowercase() == "-e" || _arg.to_lowercase() == "--edit" {
            script_arguments.drain((index - 1)..(index));
            flag = Flag::Edit;
        } else if _arg.to_lowercase() == "-c" || _arg.to_lowercase() == "--cat" {
            script_arguments.drain((index - 1)..(index));
            flag = Flag::Cat;
        } else if _arg.to_lowercase() == "-r" || _arg.to_lowercase() == "--remote" {
            flag = Flag::RemoteSsh;
            remote_address = script_arguments[index].clone();
        } else if _arg.to_lowercase().starts_with('-') {
            println!("--------------------------------------------------");
            println!("- <-- is a simbol used for flag");
            println!("Flag you are trying to use does not exist");
            println!("Do you want to proceed (not treat this symbol as a flag)?");
            println!("(y/n)");
            println!("--------------------------------------------------");
            loop {
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");

                let trimmed = input.trim();

                match trimmed.to_lowercase().as_str() {
                    "n" => {
                        std::process::exit(0);
                    }
                    "y" => {
                        break;
                    }
                    _ => {
                        println!("Invalid input. Please enter 'y' or 'n'.");
                    }
                }
            }
        }
    }
    let script_name = &script_arguments[0].clone();

    concated_script_arguments = script_arguments
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    match flag {
        Flag::None => {
            execute_command(format!(
                "{}{} {}",
                folder_path, script_name, concated_script_arguments
            ));
        }
        Flag::RemoteSsh => {
            println!("--------------------------------------------------");
            println!("Remote SSH Execution of {}", script_name);
            println!("at {}", remote_address);
            println!("--------------------------------------------------");
            execute_command(format!(
                "scp {}{} {}:~/{} ",
                folder_path, script_name, remote_address, script_name
            ));
            execute_command(format!(
                "ssh {} ./{} {}",
                remote_address, script_name, concated_script_arguments
            ));
            execute_command(format!("ssh {} rm {} ", remote_address, script_name));
        }
        Flag::Edit => {
            execute_command(format!("{} {}{}", editor, folder_path, script_name));
        }
        Flag::Cat => {
            println!("--------------------------------------------------");
            println!("Content of {} :", script_name);
            println!("--------------------------------------------------");
            execute_command(format!("cat {}{}", folder_path, script_name));
        }
    }
}
