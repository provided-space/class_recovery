use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::SystemTime;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};

use crate::jvm::parser::class_file_parser::ClassFileParser;
use crate::kmp::Haystack;
use crate::branding;

const MAGIC_VALUE: [u8; 4] = [0xCA, 0xFE, 0xBA, 0xBE];
const MAX_SIZE: usize = u16::MAX as usize;

pub fn process_bytes(buffer: &Vec<u8>, output_path: &Path) -> Result<(), String> {
    println!("Searching for Java Class pattern.");
    let indices = buffer.indices_of_needle(MAGIC_VALUE);

    println!("Found {} possible classes.\nSearching constructable classes.", indices.len());

    let now = SystemTime::now();
    let mut classes = Vec::new();
    for buffer_start in indices {
        if let Some(class) = ClassFileParser::parse(&buffer, buffer_start) {
            if class.len() > MAX_SIZE {
                continue;
            }
            classes.push(class);
        }
    }

    println!("Found {} classes in {} ms.\nWriting classes.", classes.len(), now.elapsed().unwrap().as_millis());

    let now = SystemTime::now();
    let output = match File::create(output_path) {
        Ok(output) => output,
        Err(err) => return Err(err.to_string()),
    };
    let mut archive = ZipWriter::new(output);
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);

    let mut amount_of_entries = 0;
    classes.iter().for_each(|class| {
        let _ = archive.start_file(class.get_name().to_owned() + ".class", options).and_then(|entry| {
            let _ = archive.write_all(class.get_contents());
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