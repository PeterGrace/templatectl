use anyhow::Result;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

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
    Ok(())
}
