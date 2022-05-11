use crate::config::Behavior;
use crate::record_posts::{DBInsertError, PostInDB, PostRecordRequest, PostRecordScrape};
use crate::search::Search;
use crate::utils::rand_num_wait;
use futures::executor::block_on;
use mongodb::Database;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::collections::HashSet;
use std::result::Result;
use thirtyfour::prelude::*;
use tokio;
use tokio::time::{sleep, Duration};

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq, Eq)]
pub enum PostNumber {
    First,
    Last,
    Nth(u32),
}
#[derive(Serialize, Clone, Deserialize, Debug, PartialEq, Eq)]
pub enum Action {
    PostText(TextPost),
    PostImage(ImagePost),
    LikePost(PostRetweetLike),
    SearchTwitter(Search),
    Retweet(PostRetweetLike),
    QuoteRetweet(RtQuotePost),
    CommentText(TextComment),
    CommentImage(ImageComment),
    RecordPost(PostRecorderMode),
}

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq, Eq)]
pub enum PostRecorderMode {
    Request(PostRecordRequest),
    Scrape(PostRecordScrape),
}

impl PostRecorderMode {
    pub async fn call(
        &mut self,
        db: &Database,
        driver: &WebDriver,
    ) -> Result<(), Box<DBInsertError>> {
        match self {
            PostRecorderMode::Request(object) => {
                object.post_in_db(db, driver).await?;
            }
            PostRecorderMode::Scrape(object) => {
                object.post_in_db(db, driver).await?;
            }
        }

        Ok(())
    }
}

pub trait FromText {
    fn from_text(txt: String) -> Self;
}

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq, Eq)]
pub struct TextPost {
    url: String,
    content: String,
}

impl FromText for TextPost {
    fn from_text(txt: String) -> Self {
        let s: TextPost = from_str(txt.as_str()).unwrap();

        s
    }
}

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq, Eq)]
pub struct ImagePost {
    path: String,
    text: Option<String>,
}

impl FromText for ImagePost {
    fn from_text(txt: String) -> Self {
        let s: ImagePost = from_str(txt.as_str()).unwrap();

        s
    }
}

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq, Eq)]
pub struct TextComment {
    url: String,
    text: String,
}

impl FromText for TextComment {
    fn from_text(txt: String) -> Self {
        let s: TextComment = from_str(txt.as_str()).unwrap();

        s
    }
}

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq, Eq)]
pub struct ImageComment {
    url: String,
    path: String,
    text: Option<String>,
}

impl FromText for ImageComment  {
    fn from_text(txt: String) -> Self {
        let s: ImageComment  = from_str(txt.as_str()).unwrap();

        s
    }
}


#[derive(Serialize, Clone, Deserialize, Debug, PartialEq, Eq)]
pub struct PostRetweetLike {
    url: String,
    number: PostNumber,
}

impl FromText for PostRetweetLike  {
    fn from_text(txt: String) -> Self {
        let s: PostRetweetLike  = from_str(txt.as_str()).unwrap();

        s
    }
}


#[derive(Serialize, Clone, Deserialize, Debug, PartialEq, Eq)]
pub struct RtQuotePost {
    url: String,
    number: PostNumber,
    text: Option<String>,
}

impl FromText for RtQuotePost  {
    fn from_text(txt: String) -> Self {
        let s: RtQuotePost  = from_str(txt.as_str()).unwrap();

        s
    }
}


impl Action {
    pub async fn call(
        self,
        driver: &WebDriver,
        behavior: &Behavior,
        db: &Database,
    ) -> WebDriverResult<()> {
        match self.clone() {
            Action::PostText(object) => self.post_text(driver, object, behavior).await?,
            Action::PostImage(object) => {
                self.post_image(driver, object, behavior).await?;
            }
            Action::LikePost(object) => self.like_post(driver, object, behavior).await?,
            Action::Retweet(object) => self.retweet_post(driver, object, behavior).await?,
            Action::QuoteRetweet(object) => {
                self.quote_retweet_post(driver, object, behavior).await?;
            }
            Action::CommentText(object) => {
                self.comment_text(driver, object, behavior).await?;
            }
            Action::CommentImage(object) => {
                self.comment_image(driver, object, behavior).await?;
            }
            Action::SearchTwitter(object) => {
                self.search_site(driver, object, behavior).await?;
            }
            Action::RecordPost(object) => {
                let mut clone_object = object.clone();
                let _ = clone_object.call(db, driver).await.unwrap();
            }
        }

        Ok(())
    }

    pub async fn post_text(
        &self,
        driver: &WebDriver,
        object: TextPost,
        behavior: &Behavior,
    ) -> WebDriverResult<()> {
        driver.get(object.url).await?;

        let elem_ta = driver
            .find_element(By::XPath("//*[@data-testid = \"tweetTextarea_0\"]"))
            .await?;

        let chars = object.content.chars();

        for char in chars {
            elem_ta.send_keys(char).await?;
            sleep(Duration::from_millis(rand_num_wait().into())).await;
        }

        sleep(Duration::from_millis(behavior.run_erratic_wait().into())).await;

        let elem_btn = driver
            .find_element(By::XPath("//*[@data-testid = \"tweetButtonInline\"]"))
            .await?;

        elem_btn.click().await?;

        Ok(())
    }

    pub async fn post_image(
        &self,
        driver: &WebDriver,
        object: ImagePost,
        behavior: &Behavior,
    ) -> WebDriverResult<()> {
        let elem_input = driver
            .find_element(By::XPath("//input[@data-testid = \"fileInput\"]"))
            .await?;
        elem_input.send_keys(object.path.as_str()).await?;

        sleep(Duration::from_millis(behavior.run_erratic_wait().into())).await;

        if let Some(text) = object.text {
            let elem_ta = driver
                .find_element(By::XPath("//*[@data-testid = \"tweetTextarea_0\"]"))
                .await?;

            let chars = text.chars();

            for char in chars {
                elem_ta.send_keys(char).await?;
                sleep(Duration::from_millis(100)).await;
            }

            sleep(Duration::from_millis(behavior.run_erratic_wait().into())).await;
        }

        let elem_btn = driver
            .find_element(By::XPath("//*[@data-testid = \"tweetButtonInline\"]"))
            .await?;

        elem_btn.click().await?;

        Ok(())
    }

    pub async fn retweet_post(
        &self,
        driver: &WebDriver,
        object: PostRetweetLike,
        behavior: &Behavior,
    ) -> WebDriverResult<()> {
        driver.get(object.url).await?;

        sleep(Duration::from_millis(behavior.run_erratic_wait().into())).await;

        let elem_rt: WebElement;

        match object.number {
            PostNumber::First => {
                elem_rt = driver
                    .find_element(By::XPath("//*[@data-testid = \"retweet\"]"))
                    .await?;
            }
            PostNumber::Last => {
                elem_rt = driver
                    .find_element(By::XPath("//*[@data-testid = \"retweet\"][last()]"))
                    .await?;
            }
            PostNumber::Nth(num) => {
                elem_rt = driver
                    .find_element(By::XPath(
                        format!("(//*[@data-testid = \"retweet\"])[{}]", num).as_str(),
                    ))
                    .await?;
            }
        }

        elem_rt.click().await?;

        sleep(Duration::from_millis(behavior.run_erratic_wait().into())).await;

        let elem_rt_confirm = driver
            .find_element(By::XPath("//*[@data-testid = \"retweetConfirm\"]"))
            .await?;

        elem_rt_confirm.click().await?;

        Ok(())
    }

    pub async fn quote_retweet_post(
        &self,
        driver: &WebDriver,
        object: RtQuotePost,
        behavior: &Behavior,
    ) -> WebDriverResult<()> {
        driver.get(object.url).await?;

        sleep(Duration::from_millis(behavior.run_erratic_wait().into())).await;

        let elem_rt: WebElement;

        match object.number {
            PostNumber::First => {
                elem_rt = driver
                    .find_element(By::XPath("//*[@data-testid = \"retweet\"]"))
                    .await?;
            }
            PostNumber::Last => {
                elem_rt = driver
                    .find_element(By::XPath("//*[@data-testid = \"retweet\"][last()]"))
                    .await?;
            }
            PostNumber::Nth(num) => {
                elem_rt = driver
                    .find_element(By::XPath(
                        format!("(//*[@data-testid = \"retweet\"])[{}]", num).as_str(),
                    ))
                    .await?;
            }
        }

        elem_rt.click().await?;

        sleep(Duration::from_millis(behavior.run_erratic_wait().into())).await;

        let elem_rt_confirm = driver
            .find_element(By::XPath("//a[@href = \"/compose/tweet/\"][last()]"))
            .await?;

        elem_rt_confirm.click().await?;

        if let Some(text) = object.text {
            let elem_rt_ta = driver
                .find_element(By::XPath("//*[@data-testid = \"tweetTextarea_0\"][last()]"))
                .await?;

            let chars = text.chars();

            for char in chars {
                elem_rt_ta.send_keys(char).await?;
                sleep(Duration::from_millis(rand_num_wait().into())).await;
            }

            sleep(Duration::from_millis(behavior.run_erratic_wait().into())).await;
        }

        let elem_btn = driver
            .find_element(By::XPath("//*[@data-testid = \"tweetButtonInline\"]"))
            .await?;

        elem_btn.click().await?;

        Ok(())
    }

    pub async fn like_post(
        &self,
        driver: &WebDriver,
        object: PostRetweetLike,
        behavior: &Behavior,
    ) -> WebDriverResult<()> {
        driver.get(object.url).await?;

        sleep(Duration::from_millis(behavior.run_erratic_wait().into())).await;

        let elem_like: WebElement;

        match object.number {
            PostNumber::First => {
                elem_like = driver
                    .find_element(By::XPath("//*[@data-testid = \"like\"]"))
                    .await?;
            }
            PostNumber::Last => {
                elem_like = driver
                    .find_element(By::XPath("//*[@data-testid = \"like\"][last()]"))
                    .await?;
            }
            PostNumber::Nth(num) => {
                elem_like = driver
                    .find_element(By::XPath(
                        format!("(//*[@data-testid = \"like\"])[{}]", num).as_str(),
                    ))
                    .await?;
            }
        }

        elem_like.click().await?;

        Ok(())
    }

    pub async fn comment_text(
        &self,
        driver: &WebDriver,
        object: TextComment,
        behavior: &Behavior,
    ) -> WebDriverResult<()> {
        driver.get(object.url).await?;

        sleep(Duration::from_millis(behavior.run_erratic_wait().into())).await;

        let elem_ta = driver
            .find_element(By::XPath("//*[@data-testid = \"tweetTextarea_0\"][last()]"))
            .await?;

        let chars = object.text.chars();

        for char in chars {
            elem_ta.send_keys(char).await?;
            sleep(Duration::from_millis(rand_num_wait().into())).await;
        }

        sleep(Duration::from_millis(behavior.run_erratic_wait().into())).await;

        let elem_btn = driver
            .find_element(By::XPath("//*[@data-testid = \"tweetButtonInline\"]"))
            .await?;

        elem_btn.click().await?;

        Ok(())
    }

    pub async fn comment_image(
        &self,
        driver: &WebDriver,
        object: ImageComment,
        behavior: &Behavior,
    ) -> WebDriverResult<()> {
        driver.get(object.url).await?;

        sleep(Duration::from_millis(behavior.run_erratic_wait().into())).await;

        let elem_input = driver
            .find_element(By::XPath("//input[@data-testid = \"fileInput\"]"))
            .await?;
        elem_input.send_keys(object.path.as_str()).await?;

        sleep(Duration::from_millis(behavior.run_erratic_wait().into())).await;

        if let Some(text) = object.text {
            let elem_ta = driver
                .find_element(By::XPath("//*[@data-testid = \"tweetTextarea_0\"]"))
                .await?;

            let chars = text.chars();

            for char in chars {
                elem_ta.send_keys(char).await?;
                sleep(Duration::from_millis(100)).await;
            }

            sleep(Duration::from_millis(behavior.run_erratic_wait().into())).await;
        }

        let elem_btn = driver
            .find_element(By::XPath("//*[@data-testid = \"tweetButtonInline\"]"))
            .await?;

        elem_btn.click().await?;

        Ok(())
    }

    pub async fn search_site(
        &self,
        driver: &WebDriver,
        object: Search,
        behavior: &Behavior,
    ) -> WebDriverResult<Vec<String>> {
        let url = object.format_url();

        driver.get(url).await?;

        sleep(Duration::from_millis(behavior.run_erratic_wait().into())).await;

        let mut hrefs = HashSet::<String>::new();

        loop {
            driver
            .execute_script(
                r#"
                var elSignUp = $x("//a[contains(@href, 'signup')]");
                let added = [];
                
                setInterval(() => {
                    window.scroll(0, elSignUp[0].getBoundingClientRect().top + window.scrollY * 2);
                
                    let links_snapshot = document.evaluate("//a[contains(@href, 'status')]", document, null, XPathResult.ORDERED_NODE_SNAPSHOT_TYPE, null)
                
                
                
                    for (var i = 0; i < links_snapshot.snapshotLength; i++) {
                        var node = links_snapshot.snapshotItem(i);
                
                
                
                        if (!added.includes(node.href)) {
                            let pEl = document.createElement('a');
                            pEl.setAttribute("id", `${i}-hrefStatus`)
                            pEl.setAttribute('href', node.href);
                            document.getElementsByTagName('body')[0].appendChild(pEl);
                
                            added.push(node.href);
                
                        }
                
                
                
                    }
                
                }, Math.random() * (5000 - 3000) + 3000)
                "#,
            )
            .await?;

            sleep(Duration::from_millis(1000)).await;

            driver
                .find_elements(By::XPath("//a[contains(@id, 'hrefStatus')]"))
                .await?
                .iter()
                .for_each(|x| {
                    if let Some(href) = block_on(x.get_attribute("href")).unwrap() {
                        hrefs.insert(href);
                    }
                });

            if hrefs.len() == 100 {
                break;
            }
        }

        let urls: Vec<_> = hrefs.into_iter().collect();

        Ok(urls)
    }
}
