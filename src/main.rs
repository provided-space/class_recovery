use std::fs::File;
use std::io::Read;
use std::path::Path;
use inquire::Text;

mod jvm;
mod branding;
mod core;
mod kmp;

fn main() -> Result<(), String> {
    let prompt = Text::new("Input file location")
        .with_help_message("Where is the .DMP file located at?")
        .prompt();
    let input_path = match prompt {
        Ok(path) => path,
        Err(_) => return Err("Couldn't prompt for input path.".to_owned()),
    };

    let prompt = Text::new("Where do you want to save the archive to?")
        .with_help_message("Where do you want to have the archive saved to?")
        .with_default((input_path.clone() + ".jar").as_str())
        .prompt();
    let output_path = match prompt {
        Ok(path) => path,
        Err(_) => return Err("Couldn't prompt for output path.".to_owned()),
    };

    let mut blacklist = Vec::new();
    loop {
        let prompt = Text::new("Add to blacklist")
            .with_help_message("Entries to be skipped (eg. java/, jdk/, ...). Leave blank to submit.")
            .prompt();
        match prompt {
            Ok(entry) => {
                if entry.is_empty() {
                    break;
                }
                blacklist.push(entry);
            }
            Err(_) => return Err("Couldn't prompt for blacklist entry.".to_owned()),
        };
    }

    return process(input_path, output_path, &blacklist);
}

fn process(input_path: String, output_path: String, blacklist: &Vec<String>) -> Result<(), String> {
    let mut file = match File::open(input_path) {
        Ok(file) => file,
        Err(err) => return Err(err.to_string()),
    };

    let mut buffer = Vec::new();

    println!("Reading file.");
    if file.read_to_end(&mut buffer).is_err() {
        return Err("Failed to read file buffer".to_owned());
    }
    let buffer = buffer; // make buffer immutable which could be an advantage for multi-threading

    return core::process_bytes(&buffer, Path::new(&output_path), blacklist);
}
