use crate::search::Search;
use thirtyfour::prelude::*;
use tokio;
use crate::cookie::Cookie;
use std::path::Path;
use futures::executor::block_on;
use tokio::time::{sleep, Duration};

pub enum Action {
    PostText(String),
    PostImage(ImagePost),
    LikePost(String),
    SearchTwitter(Search),
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


impl Action {
    pub async fn call(&self) -> WebDriverResult<()>  {
    }

    pub async fn post_text(&self, wd: WebDriver, ) -> WebDriverResult<()>  {

    }
}