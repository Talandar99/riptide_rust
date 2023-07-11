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

    let output = Command::new(folder_path.to_owned() + application_name)
        .output()
        .expect("");

    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        println!("{}{}", config.info_header.ok_msg, output_str);
    } else {
        let error_str = String::from_utf8_lossy(&output.stderr);
        println!("{}{}", config.info_header.fail_msg, error_str);
    }
}
