use std::fs;

fn main() {
    let folder_path = "/home/talandar/workspace/riptide/scripts";
    let entries = match fs::read_dir(folder_path) {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!("Błąd: {}", err);
            return;
        }
    };

    let mut entry_names = Vec::new();
    for entry in entries {
        if let Ok(entry) = entry {
            if let Some(entry_name) = entry.file_name().to_str() {
                entry_names.push(entry_name.to_string());
            }
        }
    }

    let subcommands = entry_names;
    let args: Vec<String> = std::env::args().collect();
    let word_being_completed = &args[2];
    for subcommand in subcommands {
        if subcommand.starts_with(word_being_completed) {
            println!("{}", subcommand);
        }
    }
}
