use crossbeam_channel::{unbounded, Receiver, Sender};
use std::cell::RefCell;
use tokio::sync::Mutex;
use tokio::task;

use mongodb::Database;
use thirtyfour::WebDriver;

use crate::{config::Behavior, cronueue::CronueueAction};

#[derive(Clone)]
pub struct CronChannel {
    name: String,
    cronueue_action: CronueueAction,
    sender: Sender<u32>,
    receiver: Receiver<u32>,
}

impl CronChannel {
    pub fn new(name_raw: String, cronueue_action: CronueueAction) -> Self {
        let (tx, rx) = unbounded();

        let name_clone = name_raw.clone();
        let mut name = name_clone;
        name.push_str("-queue");

        CronChannel {
            name,
            cronueue_action,
            sender: tx,
            receiver: rx,
        }
    }

    #[tokio::main]
    pub async fn launch(
        this: Mutex<Self>,
        driver: Mutex<WebDriver>,
        behavior: Mutex<Behavior>,
        db: Mutex<Database>,
    ) {
        task::spawn(async move {
            let self_ = this.lock().await;

            self_
                .cronueue_action
                .run_queue(driver, behavior, db, &self_.receiver)
                .await
                .unwrap();
        });
    }

    pub fn suspend(&self, millis: u32) {
        self.sender.send(millis).unwrap();
    }

    pub fn terminate(&self) {
        self.sender.send(0).unwrap();
    }
}

#[derive(Clone)]
pub struct ReadWriteQueue(RefCell<Vec<CronChannel>>);

impl ReadWriteQueue {
    pub fn new() -> Self {
        let queue = Vec::<CronChannel>::new();
        let cell = RefCell::new(queue);

        ReadWriteQueue(cell)
    }

    pub fn add_new_action(&self, name: String, action: CronueueAction) {
        let chan_cron = CronChannel::new(name, action);

        let ReadWriteQueue(cell) = self;

        let mut w = cell.borrow_mut();

        w.push(chan_cron);
    }

    pub fn launch_lastest(
        &self,
        driver: Mutex<WebDriver>,
        behavior: Mutex<Behavior>,
        db: Mutex<Database>,
    ) {
        let ReadWriteQueue(cell) = self;

        let r = cell.borrow();

        if let Some(last) = r.clone().into_iter().last() {
            let this = Mutex::new(last);

            CronChannel::launch(this, driver, behavior, db);
        }
    }

    pub async fn launch_name(
        &self,
        name: String,
        driver: Mutex<WebDriver>,
        behavior: Mutex<Behavior>,
        db: Mutex<Database>,
    ) {
        let ReadWriteQueue(cell) = self;

        let r = cell.borrow();

        for w in r.clone().into_iter().next() {
            if w.name == name {
                let this = Mutex::new(w);

                CronChannel::launch(this, driver, behavior, db);

                break;
            }
        }
    }

    pub fn suspend_latest(self, millis: u32) {
        let ReadWriteQueue(cell) = self;

        let r = cell.borrow();

        if let Some(last) = r.clone().into_iter().last() {
            last.suspend(millis);
        }
    }

    pub fn suspend_name(&'static self, name: String, millis: u32) {
        let ReadWriteQueue(cell) = self;

        let r = cell.borrow();

        for w in r.clone().into_iter() {
            if w.name == name {
                w.suspend(millis);

                break;
            }
        }
    }

    pub fn terminate_latest(&self) {
        let ReadWriteQueue(cell) = self;

        let r = cell.borrow();

        if let Some(last) = r.clone().into_iter().last() {
            last.terminate();
        }
    }

    pub fn terminate_name(&self, name: String) {
        let ReadWriteQueue(cell) = self;

        let r = cell.borrow();

        for w in r.clone().into_iter() {
            if w.name == name {
                w.terminate();

                break;
            }
        }
    }
}
