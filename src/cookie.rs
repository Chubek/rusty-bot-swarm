use crate::utils::read_from_file;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::default::Default;

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: Option<String>,
    pub path: Option<String>,
    pub expires: Option<String>,
    pub httpOnly: Option<bool>,
    pub secure: Option<bool>,
}

impl Cookie {
    pub fn from_file(fpath: &str) -> Vec<Self> {
        let contents = read_from_file(fpath).unwrap();

        let cookie: Vec<Cookie> = from_str(contents.as_str()).unwrap();

        cookie
    }

    pub fn from_string(str: &str) -> Vec<Self> {
        let cookie: Vec<Cookie> = from_str(str).unwrap();

        cookie
    }
}
