use crate::cookie::Cookie;
use rand::{self, Rng};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::default::Default;
use std::iter::Product;
use thirtyfour::{error::WebDriverResult, WebDriver};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Erracy {
    SuperErratic,
    Erratic,
    Normal,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Behavior {
    erratic_scroll: Erracy,
    erratic_wait: Erracy,
    erratic_reload: Erracy,
    wait_rng_min: u8,
    wait_rng_max: u8,
}

impl Behavior {
    pub async fn run_erratic_scroll(&self, wd: &WebDriver) -> WebDriverResult<()> {
        match self.clone().erratic_scroll {
            Erracy::SuperErratic => {
                wd.execute_script(
                    r#"
                setInterval(() => {
                    window.scroll(0, Math.random() * window.innerHeight);
                }, 10000)
            
            "#,
                )
                .await?;
            }
            Erracy::Erratic => {
                wd.execute_script(
                    r#"
                setInterval(() => {
                    window.scroll(0, Math.random() * window.innerHeight);
                }, 40000)
            
            "#,
                )
                .await?;
            }
            Erracy::Normal => {
                wd.execute_script(
                    r#"
            setInterval(() => {
                window.scroll(0, Math.random() * window.innerHeight);
            }, 80000)
        
        "#,
                )
                .await?;
            }
        }

        Ok(())
    }

    pub fn run_erratic_wait(&self) -> u8 {
        match self.clone().erratic_wait {
            Erracy::SuperErratic => {
                rand::thread_rng().gen_range(self.wait_rng_min - 10..self.wait_rng_max + 10)
            }
            Erracy::Erratic => {
                rand::thread_rng().gen_range(self.wait_rng_min - 5..self.wait_rng_max + 5)
            }
            Erracy::Normal => rand::thread_rng().gen_range(self.wait_rng_min..self.wait_rng_max),
        }
    }

    pub async fn run_erratic_reload(&self, wd: &WebDriver) -> WebDriverResult<()> {
        match self.clone().erratic_scroll {
            Erracy::SuperErratic => {
                wd.execute_script(
                    r#"
                setInterval(() => {
                    location.reload();
                }, 60000)
            
            "#,
                )
                .await?;
            }
            Erracy::Erratic => {
                wd.execute_script(
                    r#"
                setInterval(() => {
                    location.reload();
                }, 120000)
            
            "#,
                )
                .await?;
            }
            Erracy::Normal => {
                wd.execute_script(
                    r#"
            setInterval(() => {
                location.reload();
            }, 220000)
        
        "#,
                )
                .await?;
            }
        }

        Ok(())
    }
}
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Config {
    cookies: Vec<Cookie>,
    behavior: Behavior,
}

impl Config {
    pub fn from_str(s: &str) -> Self {
        let config: Config = from_str(s).unwrap();

        config
    }
}
