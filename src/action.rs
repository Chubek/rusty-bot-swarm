use crate::config::Behavior;
use crate::cookie::Cookie;
use crate::search::Search;
use futures::executor::block_on;
use std::path::Path;
use thirtyfour::prelude::*;
use tokio;
use tokio::time::{sleep, Duration};

pub enum Action<'a> {
    PostText(String),
    PostImage(ImagePost),
    LikePost(String),
    SearchTwitter(Search<'a>),
    Retweet(String),
    QuoteRetweet(RtQuote),
    CommentText(TextComment),
    CommentImage(ImageComment),
}

pub struct ImagePost {
    path: String,
    text: Option<String>,
}

pub struct RtQuote {
    url: String,
    text: Option<String>,
}

pub struct TextComment {
    url: String,
    text: String,
}

pub struct ImageComment {
    url: String,
    path: String,
    text: Option<String>,
}

impl<'a> Action<'a> {
    pub async fn call(&self, driver: &WebDriver, behavior: Behavior) -> WebDriverResult<()> {
        Ok(())
    }

    pub async fn post_text(
        &self,
        driver: &WebDriver,
        text: String,
        behavior: Behavior,
    ) -> WebDriverResult<()> {
        let elem_ta = driver
            .find_element(By::XPath("//*[@data-testid = \"tweetTextarea_0\"]"))
            .await?;

        let chars = text.chars();

        for char in chars {
            elem_ta.send_keys(char).await?;
            sleep(Duration::from_millis(100)).await;
        }

        sleep(Duration::from_millis(behavior.run_erratic_wait().into())).await;

        let elem_btn = driver
            .find_element(By::XPath("//*[@data-testid = \"tweetButtonInline\"]"))
            .await?;

        elem_btn.click().await?;

        Ok(())
    }
}
