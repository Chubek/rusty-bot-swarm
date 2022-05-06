
use crate::config::{Config, Behavior};
use crate::proxy::Proxy;
use mongodb::Database;
use thirtyfour::WebDriver;
use tokio;
use crate::read_write_queue::ReadWriteQueue;
use tokio::sync::Mutex;

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


}
