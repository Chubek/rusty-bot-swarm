use std::sync::{Arc, Mutex};
use crate::{action::Action, config::Behavior};
use chrono::prelude::*;
use mongodb::Database;
use serde::{Deserialize, Serialize};
use thirtyfour::{prelude::WebDriverResult, WebDriver};
use std::thread;
use std::time::Duration;
use crossbeam_channel::{Receiver};

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq, Eq)]
pub enum ExecType {
    Once,
    Multiple(u32),
    Forever,
}

#[derive(Serialize, Clone, Deserialize, Debug, PartialEq, Eq)]
pub struct CronueueAction {
    exec_time: DateTime<Utc>,
    action: Action,
    exec_type: ExecType,
}

impl CronueueAction {
    pub fn new(exec_time: DateTime<Utc>, action: Action, exec_type: ExecType) -> Self {
        CronueueAction {
            exec_time,
            action,
            exec_type,
        }
    }

    pub async fn run_queue(
        &self,
        driver_arc_mutex: Arc<Mutex<&WebDriver>>,
        behavior_arc_mutex: Arc<Mutex<&Behavior>>,
        db_arc_mutex: Arc<Mutex<&Database>>,
        receiver_arc_mutex: Arc<Mutex<&Receiver<u32>>>
    ) -> WebDriverResult<()> {
        let mut times_ran = 0u32;

        let driver = *driver_arc_mutex.lock().unwrap();
        let behavior = *behavior_arc_mutex.lock().unwrap();
        let db = *db_arc_mutex.lock().unwrap();
        let receiver = *receiver_arc_mutex.lock().unwrap();

        loop {
            match receiver.recv() {
                Ok(u32_sent) => {
                    if u32_sent == 0 {
                        break;
                    }
                    else {
                        thread::sleep(Duration::from_millis(u32_sent.into()));
                    }
                }
                Err(_) => panic!("Problem with sent data")
            }


            let time_now = Utc::now();

            if time_now == self.exec_time {
                match self.exec_type {
                    ExecType::Once => {
                        self.action.clone().call(driver, behavior, db).await?;
                        break;
                    }
                    ExecType::Multiple(num) => {
                        self.action.clone().call(driver, behavior, db).await?;
                        times_ran += 1;

                        if times_ran == num {
                            break;
                        }
                    }
                    ExecType::Forever => {
                        self.action.clone().call(driver, behavior, db).await?;
                    }
                }
            }            
        }

        Ok(())
    }
}
