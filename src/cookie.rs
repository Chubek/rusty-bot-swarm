use crate::utils::read_from_file;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use thirtyfour::common::cookie::Cookie as TFCookie;
use thirtyfour::{error::WebDriverResult, WebDriver};

#[derive(Serialize, Clone, Deserialize, Default, Debug, PartialEq, Eq)]
#[allow(non_snake_case)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: Option<String>,
    pub path: Option<String>,
    pub expires: Option<i64>,
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

    fn convert_cookie(self) -> TFCookie {
        let mut tf_cookie: TFCookie =
            TFCookie::new(self.name.as_str(), serde_json::json!(self.value.as_str()));

        if let Some(_) = self.domain {
            tf_cookie.set_domain(self.domain);
        }

        if let Some(_) = self.path {
            tf_cookie.set_path(self.path);
        }

        if let Some(_) = self.secure {
            tf_cookie.set_secure(self.secure);
        }

        tf_cookie
    }

    pub async fn add_all_cookies(wd: &WebDriver, cookies: Vec<Self>) -> WebDriverResult<()> {
        for cookie in cookies {
            let tf_cookie: TFCookie = cookie.convert_cookie();

            wd.add_cookie(tf_cookie).await?;
        }

        Ok(())
    }

    pub async fn load_from_str_and_add(str: &str, wd: &WebDriver) -> WebDriverResult<()> {
        let cookies = Self::from_string(str);

        Self::add_all_cookies(wd, cookies).await?;

        Ok(())
    }

    pub async fn load_from_file_and_add(floc: &str, wd: &WebDriver) -> WebDriverResult<()> {
        let cookies = Self::from_file(floc);

        Self::add_all_cookies(wd, cookies).await?;

        Ok(())
    }
}
