use thirtyfour::prelude::*;
use serde::{Deserialize, Serialize};
use tokio::time::{sleep, Duration};


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


pub struct PostRecordScrape {
    profile_url: String,
    record_mode: RecordMode,
    tweet_type: TweetType,
}


impl PostRecordScrape {
    pub fn new(profile_url: String, record_mode: RecordMode, tweet_type: TweetType) -> Self {
        PostRecordScrape {profile_url, record_mode, tweet_type}
    }

    pub async fn get_posts(&self, driver: &WebDriver) -> WebDriverResult<Vec<String>> {
        driver.get(self.profile_url.clone()).await?;
        sleep(Duration::from_millis(8000)).await;

        let scrolldown_script = r#"
            setInterva(() => {
                window.scroll(0, Math.random() * window.innerHeight);
            }, 100)
        "#;

        sleep(Duration::from_millis(500)).await;
        
        let mut posts = Vec::<String>::new();

        let has_pinned: bool;

        match driver.find_element(By::XPath("//span[text() = \"Pinned Tweet\"]")).await {
            Ok(_) => {
                has_pinned = true;
            },
            Err(_) => {
                has_pinned = false; 
            },
        }

        let mut href_click_text = String::new();
        let mut username = String::new();
        if let Some(un )= self.profile_url.split("/").last() {
            username = un.to_string();

        }


        match self.tweet_type {
            TweetType::Reply => {
                href_click_text = format!("/{}/with_replies", username);
            },
            TweetType::Post => {
                href_click_text = format!("/{}", username);
            },
            TweetType::Media => {
                href_click_text = format!("/{}/media", username);
            },
            TweetType::Likes => {
                href_click_text = format!("/{}/likes", username);
            },
        }

        let link = driver.find_element(By::XPath(
            format!("//a[@href = {}]", href_click_text).as_str()
        )).await?;

        link.click().await?;

        sleep(Duration::from_millis(300)).await;

        let links = driver.find_elements(By::XPath(
            "//a[contains(@href, \"status\")]")).await?;
        
        match self.record_mode {
            RecordMode::Last => {
                if let Some(link) = links[0].get_attribute("href").await? {
                    posts.push(link);

                }
            },
            RecordMode::LastFive => {
                if links.len() < 5 {
                    panic!("Not enough posts!")
                }

                for i in 0..5 {
                    if let Some(link) = links[i].get_attribute("href").await? {
                        posts.push(link);
    
                    }
                }
            },
            RecordMode::LastTen => {
                if links.len() < 10 {
                    panic!("Not enough posts!")
                }

                for i in 0..10 {
                    if let Some(link) = links[i].get_attribute("href").await? {
                        posts.push(link);
    
                    }
                }
            },
            RecordMode::AllFound => {
                for l in links {
                    if let Some(link) = link.get_attribute("href").await? {
                        posts.push(link);
                    }
                }
            },
        }


        Ok(posts)

    }
}