use crate::config::Behavior;
use crate::cookie::Cookie;
use crate::search::Search;
use futures::executor::block_on;
use std::path::Path;
use thirtyfour::prelude::*;
use tokio;
use tokio::time::{sleep, Duration};
use crate::utils::rand_num_wait;

pub enum PostNumber {
    First,
    Last,
    Nth(u32),
}


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

pub struct ImagePost {
    path: String,
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

pub struct PostRetweetLike {
    url: String,
    number: PostNumber,
}


pub struct RtQuotePost {
    url: String,
    number: PostNumber,
    text: Option<String>,
}

impl Action {
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
            sleep(Duration::from_millis(rand_num_wait().into())).await;
        }

        sleep(Duration::from_millis(behavior.run_erratic_wait().into())).await;

        let elem_btn = driver
            .find_element(By::XPath("//*[@data-testid = \"tweetButtonInline\"]"))
            .await?;

        elem_btn.click().await?;

        Ok(())
    }

    pub async fn post_image(&self, driver: &WebDriver, object: ImagePost, behavior: Behavior) -> WebDriverResult<()> {
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

    pub async fn retweet_post(&self, driver: &WebDriver, object: PostRetweetLike, behavior: Behavior) -> WebDriverResult<()>  {
        driver.get(object.url).await?;

        sleep(Duration::from_millis(behavior.run_erratic_wait().into())).await;
        
        let elem_rt: WebElement;
        
        match object.number {
            PostNumber::First => {
                elem_rt = driver
                    .find_element(By::XPath("//*[@data-testid = \"retweet\"]"))
                    .await?;
            },
            PostNumber::Last => {
                elem_rt = driver
                    .find_element(By::XPath("//*[@data-testid = \"retweet\"][last()]"))
                    .await?;

            },
            PostNumber::Nth(num) => {
                elem_rt = driver
                    .find_element(By::XPath(format!("(//*[@data-testid = \"retweet\"])[{}]", num).as_str()))
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

    pub async fn quote_retweet_post(&self, driver: &WebDriver, object: RtQuotePost, behavior: Behavior) -> WebDriverResult<()>  {
        driver.get(object.url).await?;

        sleep(Duration::from_millis(behavior.run_erratic_wait().into())).await;
        
        let elem_rt: WebElement;
        
        match object.number {
            PostNumber::First => {
                elem_rt = driver
                    .find_element(By::XPath("//*[@data-testid = \"retweet\"]"))
                    .await?;
            },
            PostNumber::Last => {
                elem_rt = driver
                    .find_element(By::XPath("//*[@data-testid = \"retweet\"][last()]"))
                    .await?;

            },
            PostNumber::Nth(num) => {
                elem_rt = driver
                    .find_element(By::XPath(format!("(//*[@data-testid = \"retweet\"])[{}]", num).as_str()))
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

    pub async fn like_post(&self, driver: &WebDriver, object: PostRetweetLike , behavior: Behavior) -> WebDriverResult<()>  {
        driver.get(object.url).await?;

        sleep(Duration::from_millis(behavior.run_erratic_wait().into())).await;
        
        let elem_like: WebElement;
        
        match object.number {
            PostNumber::First => {
                elem_like = driver
                    .find_element(By::XPath("//*[@data-testid = \"like\"]"))
                    .await?;
            },
            PostNumber::Last => {
                elem_like = driver
                    .find_element(By::XPath("//*[@data-testid = \"like\"][last()]"))
                    .await?;

            },
            PostNumber::Nth(num) => {
                elem_like = driver
                    .find_element(By::XPath(format!("(//*[@data-testid = \"like\"])[{}]", num).as_str()))
                    .await?;
            }
        }
        
        elem_like.click().await?;

        Ok(())
    }

    pub async fn comment_text(&self, driver: &WebDriver, object: TextComment , behavior: Behavior) -> WebDriverResult<()> {
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

    pub async fn comment_image(&self, driver: &WebDriver, object: ImageComment , behavior: Behavior) -> WebDriverResult<()> {
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

    pub async fn search_site(&self, driver: &WebDriver, object: Search, behavior: Behavior) -> WebDriverResult<Vec<String>> {
        let url = object.format_url();

        driver.get(url).await?;

        sleep(Duration::from_millis(behavior.run_erratic_wait().into())).await;


        let post_urls = driver.find_elements(
            By::XPath("//a[contains(@href, \"status\")]")
        ).await?;

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
