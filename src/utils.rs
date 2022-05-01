use chrono::{DateTime, NaiveDateTime, Utc};
use rand::{self, Rng};
use std::fs::read_to_string;
use std::fs::File;
use std::io::prelude::*;
use zip::write::FileOptions;

pub fn write_to_file(fname: &str, message: String) -> std::io::Result<()> {
    let mut file = File::create(fname)?;
    file.write_all(message.as_bytes())?;
    Ok(())
}

pub fn read_from_file(fname: &str) -> std::io::Result<String> {
    let contents = read_to_string(fname)?;

    Ok(contents)
}

pub fn convert_timestamp(timestamp: i64) -> DateTime<Utc> {
    let naive = NaiveDateTime::from_timestamp(timestamp, 0);

    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);

    datetime
}

pub fn rand_num_wait() -> u8 {
    rand::thread_rng().gen_range(120..255)
}

pub fn write_strings_to_zip(
    filename: String,
    background: String,
    manifest: String,
) -> zip::result::ZipResult<()> {
    let path = std::path::Path::new(filename.as_str());
    let file = std::fs::File::create(&path).unwrap();

    let mut zip = zip::ZipWriter::new(file);

    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o755);

    zip.start_file(format!("background.js"), options)?;
    zip.write_all(background.as_bytes())?;

    zip.start_file(format!("manifest.json"), options)?;
    zip.write_all(manifest.as_bytes())?;

    Ok(())
}
