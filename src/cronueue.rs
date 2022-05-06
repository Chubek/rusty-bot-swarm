use crate::{action::Action, config::Behavior};
use chrono::prelude::*;
use crossbeam_channel::Receiver;
use mongodb::Database;
use serde::{Deserialize, Serialize};
use std::mem::drop;
use std::time::Duration;
use thirtyfour::{prelude::WebDriverResult, WebDriver};
use tokio::{sync::Mutex, time::sleep};

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
        driver_arc_mutex: Mutex<WebDriver>,
        behavior_arc_mutex: Mutex<Behavior>,
        db_arc_mutex: Mutex<Database>,
        receiver: &Receiver<u32>,
    ) -> WebDriverResult<()> {
        let mut times_ran = 0u32;

        loop {
            match receiver.recv() {
                Ok(u32_sent) => {
                    if u32_sent == 0 {
                        break;
                    } else {
                        sleep(Duration::from_millis(u32_sent.into())).await;
                    }
                }
                Err(_) => panic!("Bad cross-thread message"),
            }

            let time_now = Utc::now();

            if time_now == self.exec_time {
                match self.exec_type {
                    ExecType::Once => {
                        let driver = driver_arc_mutex.lock().await;
                        let behavior = behavior_arc_mutex.lock().await;
                        let db = db_arc_mutex.lock().await;

                        self.action.clone().call(&driver, &behavior, &db).await?;

                        drop(driver);
                        drop(behavior);
                        drop(db);

                        break;
                    }
                    ExecType::Multiple(num) => {
                        let driver = driver_arc_mutex.lock().await;
                        let behavior = behavior_arc_mutex.lock().await;
                        let db = db_arc_mutex.lock().await;

                        self.action.clone().call(&driver, &behavior, &db).await?;
                        times_ran += 1;

                        drop(driver);
                        drop(behavior);
                        drop(db);

                        if times_ran == num {
                            break;
                        }
                    }
                    ExecType::Forever => {
                        let driver = driver_arc_mutex.lock().await;
                        let behavior = behavior_arc_mutex.lock().await;
                        let db = db_arc_mutex.lock().await;

                        self.action.clone().call(&driver, &behavior, &db).await?;

                        drop(driver);
                        drop(behavior);
                        drop(db);
                    }
                }
            }
        }

        Ok(())
    }
}
