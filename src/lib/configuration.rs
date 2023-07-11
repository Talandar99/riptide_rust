// Twój kod konfiguracji
use serde_derive::Deserialize;
use std::fs;
use std::process::exit;
use toml;

#[derive(Deserialize)]
pub struct Data {
    path: Path,
    info_header: InfoHeader,
}

#[derive(Deserialize)]
pub struct Path {
    path: String,
}

#[derive(Deserialize)]
pub struct InfoHeader {
    show_if_fail: bool,
    show_if_ok: bool,
    fail_msg: String,
    ok_msg: String,
}

pub fn get_confuguration() -> Data {
    let filename = "conf.toml";
    let contents = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read file `{}`", filename);
            exit(1);
        }
    };

    let data: Data = match toml::from_str(&contents) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Unable to load data from `{}`", filename);
            exit(1);
        }
    };
    //println!("{}", data.path.path); // => 42.69.42.0
    //println!("{}", data.info_header.show_if_fail); // => 42
    //println!("{}", data.info_header.show_if_ok); // => 42
    //println!("{}", data.info_header.fail_msg); // => 42
    //println!("{}", data.info_header.ok_msg); // => 42
    return data;
}
