#![allow(dead_code)]

#[macro_use]
extern crate lazy_static;

mod action;
mod bot;
mod config;
mod cookie;
mod cronueue;
mod proxy;
mod record_posts;
mod search;
mod utils;
mod read_write_queue;

#[cfg(test)]
mod tests {
    use crate::cookie;
    use crate::utils::write_to_file;
    use std::default::Default;
    use std::fs::remove_file;

    #[test]
    fn test_cookies_str() {
        let should_be = vec![
            cookie::Cookie {
                name: String::from("cookie1"),
                value: String::from("cookie11"),
                ..Default::default()
            },
            cookie::Cookie {
                name: String::from("cookie2"),
                value: String::from("cookie22"),
                httpOnly: Some(false),
                ..Default::default()
            },
        ];

        let str_to_be = r#"
            [
                {
                "name": "cookie1",
                "value": "cookie11"
            },

             {
                "name": "cookie2",
                "value": "cookie22",
                "httpOnly": false
            }
            ]
        
        "#;

        let is_and_is = cookie::Cookie::from_string(str_to_be);

        assert_eq!(is_and_is, should_be);
    }

    #[test]
    fn test_cookies_file() {
        let should_be = vec![
            cookie::Cookie {
                name: String::from("cookie1"),
                value: String::from("cookie11"),
                ..Default::default()
            },
            cookie::Cookie {
                name: String::from("cookie2"),
                value: String::from("cookie22"),
                httpOnly: Some(false),
                ..Default::default()
            },
        ];

        let str_to_be = r#"
            [
                {
                "name": "cookie1",
                "value": "cookie11"
            },

             {
                "name": "cookie2",
                "value": "cookie22",
                "httpOnly": false
            }
            ]
        
        "#;

        write_to_file("./temp.json", str_to_be.to_string()).unwrap();

        let is_and_is = cookie::Cookie::from_file("./temp.json");

        assert_eq!(is_and_is, should_be);

        remove_file("./temp.json").unwrap();
    }
}
