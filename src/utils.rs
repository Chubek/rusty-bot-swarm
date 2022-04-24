use std::fs::read_to_string;
use std::fs::File;
use std::io::prelude::*;

pub fn write_to_file(fname: &str, message: String) -> std::io::Result<()> {
    let mut file = File::create(fname)?;
    file.write_all(message.as_bytes())?;
    Ok(())
}

pub fn read_from_file(fname: &str) -> std::io::Result<String> {
    let contents = read_to_string(fname)?;

    Ok(contents)
}
