use std::env;
use std::fs;
use std::process::Command;

fn main() {
    let folder_path = "/home/talandar/workspace/riptide/".to_string();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("riptide takes script name as an argument");
        return;
    }
    let script_name = &args[1];

    let output = Command::new("sh")
        .arg(folder_path + script_name)
        .output()
        .expect("");

    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        println!("OK: {}", output_str);
    } else {
        let error_str = String::from_utf8_lossy(&output.stderr);
        println!("Error: {}", error_str);
    }
}
