use std::sync::Arc;

use crate::config::Config;
use crate::cronueue::CronueueAction;
use crate::proxy::Proxy;
use thirtyfour::prelude::WebDriverResult;
use tokio;

pub struct Bot {
    queued_actions: Vec<CronueueAction>,
    config: Config,
    proxy: Proxy,
}

impl Bot {
    pub fn new(queued_actions: Vec<CronueueAction>, config: Config, proxy: Proxy) -> Self {
        Bot {
            queued_actions,
            config,
            proxy,
        }
    }

    #[tokio::main]
    pub async fn run(&'static self) -> WebDriverResult<()> {
        let driver = self
            .proxy
            .launch_driver_with_proxy(self.config.clone())
            .await?;

        let db = self.config.create_db().await;

        let driver_arc = Arc::new(driver);
        let behavior_arc = Arc::new(self.config.behavior.clone());
        let db_arc = Arc::new(db);

        for qaction in self.queued_actions.iter() {
            let driver_arc_clone = Arc::clone(&driver_arc);
            let behavior_arc_clone = Arc::clone(&behavior_arc);
            let db_arc_clone = Arc::clone(&db_arc);

            tokio::task::spawn(qaction.run_queue(
                driver_arc_clone,
                behavior_arc_clone,
                db_arc_clone,
            ));
        }

        Ok(())
    }
}
