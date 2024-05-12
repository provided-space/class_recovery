use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::SystemTime;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};

use crate::jvm::parser::class_file_parser::ClassFileParser;
use crate::kmp::Haystack;
use crate::branding;
use crate::jvm::class_buffer::ClassBuffer;

const MAGIC_VALUE: [u8; 4] = [0xCA, 0xFE, 0xBA, 0xBE];
const MAX_SIZE: usize = u16::MAX as usize;

pub fn process_bytes(buffer: &Vec<u8>, output_path: &Path, blacklist: &Vec<String>) -> Result<(), String> {
    println!("Searching for Java Class pattern.");
    let indices = buffer.indices_of_needle(MAGIC_VALUE);

    println!("Found {} possible classes.\nSearching constructable classes.", indices.len());

    let now = SystemTime::now();
    let mut class_map: HashMap<String, Vec<ClassBuffer>> = HashMap::new();
    let mut amount_of_classes = 0;

    for buffer_start in indices {
        let class = match ClassFileParser::parse(&buffer, buffer_start, blacklist.clone()) {
            Some(class) => class,
            None => continue,
        };

        if class.len() > MAX_SIZE {
            continue;
        }

        let class_name = class.get_name().to_owned();
        let classes = class_map.entry(class_name).or_default();
        classes.push(class.clone());
        amount_of_classes += 1;
    }

    println!("Found {} classes in {} ms.\nWriting classes.", amount_of_classes, now.elapsed().unwrap().as_millis());

    let now = SystemTime::now();
    let output = match File::create(output_path) {
        Ok(output) => output,
        Err(err) => return Err(err.to_string()),
    };
    let mut archive = ZipWriter::new(output);
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);

    let mut amount_of_entries = 0;
    class_map.iter().for_each(|(_, classes)| {
        let mut i = 0;
        for class in classes {
            i += 1;

            let mut file_name = class.get_name().to_owned() + ".class";
            if i != 1 {
                file_name += format!(" ({})", i).as_str();
            }

            let _ = archive.start_file(file_name, options).and_then(|entry| {
                let _ = archive.write_all(class.get_contents());
                amount_of_entries += 1;
                return Ok(entry);
            });
        }
    });

    let comment = format!("These classes were recovered by {} ({})\nVisit {} for more information", branding::NAME, branding::VERSION, branding::REPOSITORY);
    archive.set_comment(comment);

    if archive.finish().is_err() {
        return Err("Failed to write classes to archive".to_owned());
    }
    println!("Wrote {} classes in {} ms.", amount_of_entries, now.elapsed().unwrap().as_millis());

    return Ok(());
}