use std::fs::File;
use std::{env, io};
use std::io::{Read, Write};
use std::path::Path;
use std::time::SystemTime;

use zip::{CompressionMethod, ZipWriter};
use zip::write::SimpleFileOptions;

use crate::jvm::class_file_parser::ClassFileParser;
use crate::kmp::Haystack;

mod kmp;
mod jvm;

const MAGIC_VALUE: [u8; 4] = [0xCA, 0xFE, 0xBA, 0xBE];

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).expect("Missing path to .DMP file");

    let mut buffer = Vec::new();
    let mut classes = Vec::new();

    let mut file = File::open(path)?;
    println!("Reading file.");
    file.read_to_end(&mut buffer)?;

    let buffer = buffer; // make buffer immutable which could be an advantage for multi-threading
    let mut amount_of_classes = 0;

    println!("Searching for Java Class pattern.");
    let indices = buffer.indices_of_needle(MAGIC_VALUE);

    println!("Found {} possible classes.\nSearching constructable classes.", indices.len());

    let now = SystemTime::now();
    for buffer_start in indices {
        if let Some(result) = ClassFileParser::get_end_of_class(&buffer, buffer_start) {
            classes.push((result.0, buffer_start, result.1));
            amount_of_classes += 1;
        }
    }

    println!("Found {} classes in {} ms.\nWriting classes.", amount_of_classes, now.elapsed().unwrap().as_millis());

    let now = SystemTime::now();
    let mut archive = ZipWriter::new(File::create(Path::new(&(path.to_owned() + ".jar")))?);
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);

    classes.iter().for_each(|(class_name, buffer_start, buffer_end)| {
        let _ = archive.start_file(class_name.to_owned() + ".class", options).and_then(|entry| {
            let start = *buffer_start;
            let end = *buffer_end;
            let _ = archive.write_all(&buffer[start..end]);
            return Ok(entry);
        });
    });

    archive.finish()?;
    println!("Wrote {} classes in {} ms.", amount_of_classes, now.elapsed().unwrap().as_millis());

    return Ok(());
}

