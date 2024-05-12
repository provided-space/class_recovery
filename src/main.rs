use std::fs::File;
use std::env;
use std::io::Read;
use std::path::Path;

mod jvm;
mod branding;
mod core;
mod kmp;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let path = match args.get(1) {
        Some(path) => path,
        None => return Err("Missing argument for path to .DMP file".to_owned()),
    };

    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(err) => return Err(err.to_string()),
    };

    let mut buffer = Vec::new();

    println!("Reading file.");
    if file.read_to_end(&mut buffer).is_err() {
        return Err("Failed to read file buffer".to_owned());
    }
    let buffer = buffer; // make buffer immutable which could be an advantage for multi-threading

    return core::process_bytes(&buffer, Path::new(&(path.to_owned() + ".jar")));
}
