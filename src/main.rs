use std::fs::File;
use std::env;
use std::io::{Read, Write};
use std::path::Path;
use std::time::SystemTime;

use zip::{CompressionMethod, ZipWriter};
use zip::write::SimpleFileOptions;

use jvm::parser::class_file_parser::ClassFileParser;
use crate::kmp::Haystack;

mod kmp;
mod jvm;
mod branding;

const MAGIC_VALUE: [u8; 4] = [0xCA, 0xFE, 0xBA, 0xBE];

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
    let mut classes = Vec::new();

    println!("Reading file.");
    if file.read_to_end(&mut buffer).is_err() {
        return Err("Failed to read file buffer".to_owned());
    }
    let buffer = buffer; // make buffer immutable which could be an advantage for multi-threading

    println!("Searching for Java Class pattern.");
    let indices = buffer.indices_of_needle(MAGIC_VALUE);

    println!("Found {} possible classes.\nSearching constructable classes.", indices.len());

    let now = SystemTime::now();
    for buffer_start in indices {
        if let Some(result) = ClassFileParser::get_end_of_class(&buffer, buffer_start) {
            classes.push((result.0, buffer_start, result.1));
        }
    }

    println!("Found {} classes in {} ms.\nWriting classes.", classes.len(), now.elapsed().unwrap().as_millis());

    let now = SystemTime::now();
    let output = match File::create(Path::new(&(path.to_owned() + ".jar"))) {
        Ok(output) => output,
        Err(err) => return Err(err.to_string()),
    };
    let mut archive = ZipWriter::new(output);
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);

    let mut amount_of_entries = 0;
    classes.iter().for_each(|(class_name, buffer_start, buffer_end)| {
        let _ = archive.start_file(class_name.to_owned() + ".class", options).and_then(|entry| {
            let start = *buffer_start;
            let end = *buffer_end;
            let _ = archive.write_all(&buffer[start..end]);
            amount_of_entries += 1;
            return Ok(entry);
        });
    });

    let comment = format!("These classes were recovered by {} ({})\nVisit {} for more information", branding::NAME, branding::VERSION, branding::REPOSITORY);
    archive.set_comment(comment);

    if archive.finish().is_err() {
        return Err("Failed to write classes to archive".to_owned());
    }
    println!("Wrote {} classes in {} ms.", amount_of_entries, now.elapsed().unwrap().as_millis());

    return Ok(());
}
