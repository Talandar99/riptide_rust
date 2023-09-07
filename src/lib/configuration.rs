use serde_derive::Deserialize;
use std::fs;
use std::process::exit;
use toml;

#[derive(Deserialize)]
pub struct Data {
    pub path: Path,
}

#[derive(Deserialize)]
pub struct Path {
    pub path: String,
}

pub fn get_confuguration() -> Data {
    let home_dir = match dirs::home_dir() {
        Some(path) => path,
        None => {
            println!("Can't get configuration");
            exit(1);
        }
    };
    let filename = "config.toml";

    let path = format!(
        "{}{}{}",
        home_dir.to_str().unwrap(),
        "/.config/riptide/",
        filename,
    );

    let contents = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("I am looking for config.toml file in ~\\.config\\riptide\\ \nand file does not exist :/");
            exit(1);
        }
    };

    let mut data: Data = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Unable to load data from `{}`", filename);
            exit(1);
        }
    };

    if !data.path.path.ends_with('/') {
        data.path.path = data.path.path + "/";
    }

    if let Ok(metadata) = fs::metadata(data.path.path.clone()) {
        if !metadata.is_dir() {
            println!("your \"path\" is pointing at file not directory. Add proper directory path");
            exit(1);
        }
    } else {
        println!("your \"path\" does not exist. Add proper directory path");
        exit(1);
    }
    return data;
}
