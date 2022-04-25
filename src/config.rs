use std::iter::Product;

use crate::cookie::Cookie;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use crate::utils::rand_num_betwee_oen_and_three;


#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq)]
pub enum Erracy {
    SuperErratic,
    Erratic,
    Normal,
}


#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq)]
pub struct Behavior {
    erratic_scroll: Erracy,
    erratic_wait: Erracy,
    erratic_reload: Erracy,
    wait_rng_min: u8,
    wait_rng_max: u8,
}

impl Behavior {
    pub fn return_erratic_script(&self) -> String {
        let rand_num = rand_num_betwee_oen_and_three();

        match rand_num {
            1 => {
                match self.clone().erratic_scroll {
                    Erracy::SuperErratic => {
                        String::from(r#"
                            setInterval(() => {
                                window.scroll(0, Math.random() * window.innerHeight);
                            }, 1000)
                        
                        "#)
                    },
                    Erracy::Erratic => {
                        String::from(r#"
                            setInterval(() => {
                                window.scroll(0, Math.random() * window.innerHeight);
                            }, 4000)
                        
                        "#)
                    },                    
                    Erracy::Normal => {
                        String::from(r#"
                            window.scroll(0, Math.random() * window.innerHeight);                        
                        "#)
                    },
                }
            }
        }
        

        
    }
}

pub struct Config {
    cookies: Vec<Cookie>,
    behavior: Behavior,
}