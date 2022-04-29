use std::sync::Arc;

use crate::cronueue::CronueueAction;
use crate::config::Config;
use crate::proxy::{Proxy, self};
use thirtyfour::prelude::WebDriverResult;
use tokio;


pub struct Bot {
    queued_actions: Vec<CronueueAction>,
    config: Config,
    proxy: Proxy
}


impl Bot {
    pub fn new(queued_actions: Vec<CronueueAction>, config: Config, proxy: Proxy) -> Self {
        Bot { queued_actions, config, proxy }
    }

    #[tokio::main]
    pub async fn run(&'static self) -> WebDriverResult<()> {
        let driver = self.proxy.launch_driver_with_proxy(
                        self.config.clone()).await?;

        let driver_arc = Arc::new(driver);
        let behavior_arc = Arc::new(self.config.behavior.clone());

        for qaction in self.queued_actions.iter() {
            let driver_arc_clone = Arc::clone(&driver_arc);
            let behavior_arc_clone = Arc::clone(&behavior_arc);

            tokio::task::spawn(
                qaction.run_queue(driver_arc_clone, behavior_arc_clone)
            );
        }

        Ok(())
    }
}

