use anyhow::Result;
use log::debug;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

pub fn read_file(filepath: &str) -> Result<String> {
    let file = File::open(filepath)?;
    let mut buf_reader = BufReader::new(file);
    let mut file_str = String::new();
    let _numbytes = buf_reader.read_to_string(&mut file_str)?;
    Ok(file_str)
}

pub fn write_file(filepath: &str, file_contents: String) -> Result<()> {
    let file = File::create(filepath)?;
    let mut buf_writer = BufWriter::new(file);
    buf_writer.write_all(file_contents.as_bytes())?;
    println!("Template file {} has changed.  You may need to restart xochitl for the changes to take effect.",filepath);
    Ok(())
}

pub fn is_template_file(filename: String) -> bool {
    let exists =
        Path::new(format!("/usr/share/remarkable/templates/{}", filename).as_str()).exists();
    debug!("template file exists: {}", exists);
    exists
}
