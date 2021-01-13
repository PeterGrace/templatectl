use anyhow::Result;
use std::fs::File;
use std::io::{BufReader, Read};

pub fn read_file(filepath: &str) -> Result<String> {
    let file = File::open(filepath)?;
    let mut buf_reader = BufReader::new(file);
    let mut file_str = String::new();
    let _numbytes = buf_reader.read_to_string(&mut file_str)?;
    Ok(file_str)
}
