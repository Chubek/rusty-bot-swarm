use crate::config::Behavior;
use crate::search::Search;
use crate::utils::rand_num_wait;
use serde::{Deserialize, Serialize};
use thirtyfour::prelude::*;
use tokio;
use tokio::time::{sleep, Duration};
use mongodb::{Database, bson::{Document, doc}};
use crate::utils::today_date_coll_name;


#[derive(Serialize, Clone, Deserialize, Debug, PartialEq, Eq)]
pub enum PostNumber {
    First,
    Last,
    Nth(u32),
}
#[derive(Serialize, Clone, Deserialize, Debug, PartialEq, Eq)]
pub enum Action {
    PostText(String),
    PostImage(ImagePost),
    LikePost(PostRetweetLike),
    SearchTwitter(Search),
    Retweet(PostRetweetLike),
    QuoteRetweet(RtQuotePost),
    CommentText(TextComment),
    CommentImage(ImageComment),
}
#[derive(Serialize, Clone, Deserialize, Default, Debug, PartialEq, Eq)]
pub struct ImagePost {
    path: String,
    text: Option<String>,
}

#[derive(Serialize, Clone, Deserialize, Default, Debug, PartialEq, Eq)]
pub struct TextComment {
    url: String,
    text: String,
}

#[derive(Serialize, Clone, Deserialize, Default, Debug, PartialEq, Eq)]
pub struct ImageComment {
    url: String,
    path: String,
    text: Option<String>,
}

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq, Eq)]
pub struct PostRetweetLike {
    url: String,
    number: PostNumber,
}

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq, Eq)]
pub struct RtQuotePost {
    url: String,
    number: PostNumber,
    text: Option<String>,
}

impl Action {
    pub async fn call(self, driver: &WebDriver, behavior: &Behavior) -> WebDriverResult<()> {
       
        match self.clone() {
            Action::PostText(text) => self.post_text(driver, text, behavior).await?,
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
        }

        Ok(())
    }

    pub async fn post_text(
        &self,
        driver: &WebDriver,
        text: String,
        behavior: &Behavior,
    ) -> WebDriverResult<()> {
        let elem_ta = driver
            .find_element(By::XPath("//*[@data-testid = \"tweetTextarea_0\"]"))
            .await?;

        let chars = text.chars();

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

        driver
            .execute_script(
                r#"
        setInterval(() => {
            window.scroll(0, Math.random() * window.innerHeight);
        }, 100)
    
    "#,
            )
            .await?;

        sleep(Duration::from_millis(15000)).await;

        let post_urls = driver
            .find_elements(By::XPath("//a[contains(@href, \"status\")]"))
            .await?;

        let mut urls = Vec::<String>::new();

        for post_url in post_urls {
            let url_option = post_url.get_attribute("href").await?;

            if let Some(url) = url_option {
                urls.push(url);
            }
        }

        Ok(urls)
    }
}
