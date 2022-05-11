
use crate::config::{Config, Behavior};
use crate::proxy::Proxy;
use mongodb::Database;
use thirtyfour::WebDriver;
use tokio;
use crate::read_write_queue::ReadWriteQueue;
use tokio::sync::Mutex;
use crate::action::*;
use crate::search::Search;
pub struct Bot {
    name: String,
    queue: ReadWriteQueue,
    driver: Mutex<WebDriver>,
    db: Mutex<Database>,
    behavior: Mutex<Behavior>,

}

impl Bot {
    #[tokio::main]
    pub async fn new(name_raw: String, proxy_str: String, config_str: String) -> Self {
        let name_clone = name_raw.clone();
        let mut name = name_clone;
        name.push_str("-bot");

        let proxy = Proxy::from_str(proxy_str);
        let config = Config::from_str(config_str);

        let queue = ReadWriteQueue::new();

        let driver_result = proxy
                 .launch_driver_with_proxy(config.clone()).await.unwrap();
       

        let db_result = config.clone().create_db().await;

        let db = Mutex::new(db_result);
        let driver = Mutex::new(driver_result);
        let behavior = Mutex::new(config.behavior.clone());


        Bot {
            name,
            queue,
            driver,
            db,
            behavior,
        }
    }

    pub fn create_post_action(&self, json: String) -> Action {
        let post_post = TextPost::from_text(json);

        let action = Action::PostText(post_post);

        action
    }

    pub fn create_image_action(&self, json: String) -> Action {
        let post_post = ImagePost::from_text(json);

        let action = Action::PostImage(post_post);

        action
    }


    pub fn create_like_action(&self, json: String) -> Action {
        let post_post = PostRetweetLike::from_text(json);

        let action = Action::LikePost(post_post);

        action
    }

    pub fn create_search_action(&self, json: String) -> Action {
        let post_post = Search::from_json_string(json);

        let action = Action::SearchTwitter(post_post);

        action
    }


    pub fn create_rt_action(&self, json: String) -> Action {
        let post_post = PostRetweetLike::from_text(json);

        let action = Action::Retweet(post_post);

        action
    }


    pub fn create_qrt_action(&self, json: String) -> Action {
        let post_post = RtQuotePost::from_text(json);

        let action = Action::QuoteRetweet(post_post);

        action
    }


    pub fn create_ctext_action(&self, json: String) -> Action {
        let post_post = TextComment::from_text(json);

        let action = Action::CommentText(post_post);

        action
    }



    pub fn create_cimage_action(&self, json: String) -> Action {
        let post_post = ImageComment::from_text(json);

        let action = Action::CommentImage(post_post);

        action
    }
}
