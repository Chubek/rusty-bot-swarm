use crate::utils::{make_get_post_url, today_date_coll_name};
use async_trait::async_trait;
use futures::executor::block_on;
use mongodb::bson::{doc, Document};
use mongodb::Database;
use regex::Regex;
use reqwest::header::*;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::result::Result;
use thirtyfour::prelude::*;
use tokio::time::{sleep, Duration};

lazy_static! {
    static ref RE_POST: Regex = Regex::new(r#""__typename":"Tweet","rest_id":"\d+""#).unwrap();
    static ref RE_USER_NAME: Regex = Regex::new(r#""screen_name":"([A-Za-z0-9\_]+)""#).unwrap();
}

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq, Eq)]
pub enum RecordMode {
    Last,
    LastFive,
    LastTen,
    AllFound,
}

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq, Eq)]
pub enum TweetType {
    Reply,
    Post,
    Media,
    Likes,
}

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq, Eq)]
pub struct PostRecordScrape {
    profile_url: String,
    record_mode: RecordMode,
    tweet_type: TweetType,
}

#[derive(Debug)]
pub struct DBInsertError {
    details: String,
}

impl fmt::Display for DBInsertError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for DBInsertError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl DBInsertError {
    pub fn new(details: &str) -> Box<Self> {
        let err = DBInsertError {
            details: details.to_string(),
        };

        Box::from(err)
    }
}

#[async_trait]
pub trait PostInDB {
    async fn post_in_db(
        &mut self,
        db: &Database,
        driver: &WebDriver,
    ) -> Result<(), Box<DBInsertError>>;
}

impl PostRecordScrape {
    pub fn new(profile_url: String, record_mode: RecordMode, tweet_type: TweetType) -> Self {
        PostRecordScrape {
            profile_url,
            record_mode,
            tweet_type,
        }
    }

    pub async fn get_posts(&self, driver: &WebDriver) -> WebDriverResult<Vec<String>> {
        driver.get(self.profile_url.clone()).await?;
        sleep(Duration::from_millis(8000)).await;

        let scrolldown_script = r#"
            setInterva(() => {
                window.scroll(0, Math.random() * window.innerHeight);
            }, 100)
        "#;

        driver.execute_script(scrolldown_script).await?;

        sleep(Duration::from_millis(500)).await;

        let mut posts = Vec::<String>::new();

        let has_pinned: usize;

        match driver
            .find_element(By::XPath("//span[text() = \"Pinned Tweet\"]"))
            .await
        {
            Ok(_) => {
                has_pinned = 1;
            }
            Err(_) => {
                has_pinned = 0;
            }
        }

        let href_click_text: String;
        let mut username = String::new();
        if let Some(un) = self.profile_url.split("/").last() {
            username = un.to_string();
        }

        match self.tweet_type {
            TweetType::Reply => {
                href_click_text = format!("/{}/with_replies", username);
            }
            TweetType::Post => {
                href_click_text = format!("/{}", username);
            }
            TweetType::Media => {
                href_click_text = format!("/{}/media", username);
            }
            TweetType::Likes => {
                href_click_text = format!("/{}/likes", username);
            }
        }

        let link = driver
            .find_element(By::XPath(
                format!("//a[@href = {}]", href_click_text).as_str(),
            ))
            .await?;

        link.click().await?;

        sleep(Duration::from_millis(300)).await;

        let links = driver
            .find_elements(By::XPath("//a[contains(@href, \"status\")]"))
            .await?;

        match self.record_mode {
            RecordMode::Last => {
                if let Some(link) = links[0 + has_pinned].get_attribute("href").await? {
                    posts.push(link);
                }
            }
            RecordMode::LastFive => {
                if links.len() < 5 {
                    panic!("Not enough posts!")
                }

                for i in 0..5 {
                    if let Some(link) = links[i + has_pinned].get_attribute("href").await? {
                        posts.push(link);
                    }
                }
            }
            RecordMode::LastTen => {
                if links.len() < 10 {
                    panic!("Not enough posts!")
                }

                for i in 0..10 {
                    if let Some(link) = links[i + has_pinned].get_attribute("href").await? {
                        posts.push(link);
                    }
                }
            }
            RecordMode::AllFound => {
                for l in links {
                    if let Some(link) = l.get_attribute("href").await? {
                        posts.push(link);
                    }
                }
            }
        }

        Ok(posts)
    }
}

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq, Eq)]
pub struct SearchHeader {
    x_csrf_token: String,
    cookie: String,
    authorization: String,
    x_twitter_active_user: String,
    x_twitter_auth_type: String,
    content_type: String,
    te: String,
    host: String,
    referer: String,
    accept: String,
    user_agent: String,
}

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq, Eq)]
pub struct PostRecordRequest {
    user_id: String,
    link_id: String,
    count: u32,
    json: String,
    record_mode: RecordMode,
    search_header: SearchHeader,
}

impl PostRecordRequest {
    pub fn new(
        user_id: String,
        link_id: String,
        count: u32,
        record_mode: RecordMode,
        search_header: SearchHeader,
    ) -> Self {
        PostRecordRequest {
            user_id,
            link_id,
            count,
            json: String::new(),
            record_mode,
            search_header,
        }
    }

    pub async fn get_json(&mut self) {
        let url = make_get_post_url(self.user_id.clone(), self.count, self.link_id.clone());

        let client = reqwest::Client::new();
        let res = client
            .get(url)
            .header(USER_AGENT, self.search_header.clone().user_agent)
            .header(REFERER, self.search_header.clone().referer)
            .header(ACCEPT, self.search_header.clone().accept)
            .header(CONTENT_TYPE, self.search_header.clone().content_type)
            .header(
                "x-twitter-auth-type",
                self.search_header.clone().x_twitter_auth_type,
            )
            .header(
                "x-twitter-active-user",
                self.search_header.clone().x_twitter_active_user,
            )
            .header(AUTHORIZATION, self.search_header.clone().authorization)
            .header(HOST, self.search_header.clone().host)
            .header(COOKIE, self.search_header.clone().cookie)
            .header("TE", self.search_header.clone().te)
            .header("X_CSRF_TOKEN", self.search_header.clone().x_csrf_token)
            .send()
            .await
            .unwrap();
        self.json = res.text().await.unwrap();
    }

    pub fn get_posts(&self) -> Vec<String> {
        let matches = RE_POST
            .find_iter(self.json.as_str())
            .into_iter()
            .collect::<Vec<_>>();

        let json_chars = self.json.split("").collect::<Vec<_>>();

        let mut rets = Vec::<String>::new();

        let nth: Vec<u32>;

        match self.record_mode {
            RecordMode::Last => {
                nth = vec![0];
            }
            RecordMode::LastFive => {
                nth = (0..5).collect();
            }
            RecordMode::LastTen => {
                nth = (0..10).collect();
            }
            RecordMode::AllFound => {
                let matches_vec_len = matches.clone().len();

                nth = (0..matches_vec_len as u32).collect();
            }
        }

        for n in nth {
            let mat = matches[n as usize];
            let mut st = String::new();
            for j in mat.start()..mat.end() {
                st.push_str(json_chars[j]);
            }

            st = Self::extract_numbers(st);

            rets.push(st);
        }

        rets
    }

    fn extract_numbers(str: String) -> String {
        let re_num = Regex::new(r#"\d+"#).unwrap();

        let mut post_num = String::new();

        let found = re_num.find(str.as_str()).unwrap();

        let str_split = str.split("").into_iter().collect::<Vec<_>>();

        for i in found.start()..found.end() {
            post_num.push_str(str_split[i])
        }

        post_num
    }

    fn extract_user_name(&self) -> String {
        let mat = RE_USER_NAME.find(self.json.as_str()).unwrap();

        let mut found = String::new();

        let str_split = self.json.split("").collect::<Vec<_>>();

        for i in mat.start()..mat.end() {
            found.push_str(str_split[i]);
        }

        found = found.replace("\"screen_name\"", "");
        found = found.replace("\"", "");

        found
    }
}

#[async_trait]
impl PostInDB for PostRecordScrape {
    async fn post_in_db(
        &mut self,
        db: &Database,
        driver: &WebDriver,
    ) -> Result<(), Box<DBInsertError>> {
        let collection = db.collection::<Document>(&today_date_coll_name());

        let mut user_name = String::new();

        if let Some(user_name_str) = self.profile_url.split("/").last() {
            user_name = user_name_str.to_string();
        }

        let posts = block_on(self.get_posts(&driver))
            .unwrap()
            .into_iter()
            .map(|x| {
                if let Some(ret) = x.split("/").last() {
                    return doc! {"username": user_name.clone(),
                    "post": ret.to_owned()};
                } else {
                    panic!("Problem with url");
                }
            })
            .collect::<Vec<_>>();

        if posts.len() == 0 {
            return Err(DBInsertError::new("Length of posts is 0"));
        }

        collection.insert_many(posts, None).await.unwrap();

        Ok(())
    }
}

#[async_trait]
impl PostInDB for PostRecordRequest {
    async fn post_in_db(&mut self, db: &Database, _: &WebDriver) -> Result<(), Box<DBInsertError>> {
        self.get_json().await;

        let collection = db.collection::<Document>(&today_date_coll_name());

        let user_name = self.extract_user_name();

        let posts = self
            .get_posts()
            .into_iter()
            .map(|x| {
                if let Some(ret) = x.split("/").last() {
                    return doc! {"username": user_name.clone(),
                    "post": ret.to_owned()};
                } else {
                    panic!("Problem with url");
                }
            })
            .collect::<Vec<_>>();

        if posts.len() == 0 {
            return Err(DBInsertError::new("Length of posts is 0"));
        }

        collection.insert_many(posts, None).await.unwrap();

        Ok(())
    }
}
