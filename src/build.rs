use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let file_path = Path::new("./src/branding.rs");
    let mut output_file = File::create(&file_path)?;

    let repository = std::env::var("GH_REPOSITORY").unwrap_or("-".to_owned());
    let name = "class_recovery";
    let version = std::env::var("GH_VERSION").unwrap_or("in-dev".to_owned());

    let content = format!(
        "pub const REPOSITORY: &str = \"{repository}\";\n\
        pub const NAME: &str = \"{name}\";\n\
        pub const VERSION: &str = \"{version}\";\n\
        ",
    );

    return output_file.write_all(content.as_bytes());
}
