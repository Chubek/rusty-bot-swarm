use chrono::{DateTime, NaiveDateTime, Utc};
use std::fs::read_to_string;
use std::fs::File;
use std::io::prelude::*;
use substring::Substring;
use rand::{self, Rng};

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

pub fn rand_num_betwee_oen_and_three() -> u8 {
    rand::thread_rng().gen_range(0..4)
}