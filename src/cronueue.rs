use std::sync::Arc;

use chrono::prelude::*;
use lazy_static::__Deref;
use serde::{Deserialize, Serialize};
use thirtyfour::{prelude::WebDriverResult, WebDriver};
use crate::{action::{Action, self}, config::Behavior};

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq, Eq)]
pub enum ExecType {
    Once,
    Multiple(u32),
    Forever,
}

#[derive(Serialize, Clone,  Deserialize, Debug, PartialEq, Eq)]
pub struct CronueueAction {
    exec_time: DateTime<Utc>,
    action: Action,
    exec_type: ExecType,
}

impl  CronueueAction {
    pub fn new(exec_time: DateTime<Utc>, action: Action, exec_type: ExecType) -> Self {
        CronueueAction { exec_time, action, exec_type }
    }

    pub async fn run_queue(&self, driver_arc: Arc<WebDriver>, behavior_arc: Arc<Behavior>) -> WebDriverResult<()>  {
        let mut times_ran = 0u32;
        
        let driver = driver_arc.deref();
        let behavior = behavior_arc.deref();

        loop {
            let time_now = Utc::now();

            if time_now == self.exec_time {
                match self.exec_type {
                    ExecType::Once => {
                        self.action.clone().call(driver, behavior).await?;
                        break;
                    },
                    ExecType::Multiple(num) => {
                        self.action.clone().call(driver, behavior).await?;
                        times_ran += 1;
                        
                        if times_ran == num {
                            break;
                        }
                    },
                    ExecType::Forever => {
                        self.action.clone().call(driver, behavior).await?;
                    },
                }
            }

            
        }

        Ok(())
    }
}