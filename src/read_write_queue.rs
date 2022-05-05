use std::marker::PhantomData;
use std::sync::{RwLock, Arc, Mutex};
use std::thread;
use futures::executor::block_on;
use crossbeam_channel::{unbounded, Sender, Receiver};

use mongodb::Database;
use thirtyfour::WebDriver;

use crate::{cronueue::CronueueAction, config::Behavior};

#[allow(dead_code)]
pub struct CronChannel<'a> {
    name: String,
    cronueue_action: CronueueAction,
    sender: Sender<u32>,
    receiver: Receiver<u32>,
    _phantom: PhantomData<&'a str>,
}

impl<'a> CronChannel<'a> {
    pub fn new(name_raw: String, cronueue_action: CronueueAction) -> Self {
        let (tx, rx):(Sender<u32>, Receiver<u32>) = unbounded();
        
        let _phantom = PhantomData;
        
        let name_clone = name_raw.clone();
        let mut name = name_clone;
        name.push_str("-queue");

        CronChannel { name, cronueue_action, sender: tx, receiver: rx, _phantom  }
    }


    pub fn launch(&'static self, driver: &'static WebDriver, behavior: &'static Behavior, db: &'static Database) {
        let db_arc = Arc::new(Mutex::new(db));
        let driver_arc = Arc::new(Mutex::new(driver));
        let behavior_arc = Arc::new(Mutex::new(behavior));
        let receiver_arc = Arc::new(Mutex::new(&self.receiver));
       
        thread::spawn(move || {
            block_on(self.cronueue_action.run_queue(driver_arc, behavior_arc, db_arc, receiver_arc));
        });   
    }

    pub fn suspend(&'static self, millis: u32) {
        self.sender.send(millis);
    }

    pub fn terminate(&'static self) {
        self.sender.send(0);
    }
}


pub struct ReadWriteQueue<'a>(RwLock<Arc<Vec<CronChannel<'a>>>>);

impl<'a> ReadWriteQueue<'a> {
    pub fn new() -> Self {
        let arc = Arc::new(Vec::<CronChannel<'a>>::new());
        let rwlock = RwLock::new(arc);

        ReadWriteQueue(rwlock)
    }

    pub fn add_new_action(&'static self, name: String, action: CronueueAction) {
        let chan_cron: CronChannel<'static> = CronChannel::new(name, action);

        let ReadWriteQueue(rw)  = self;

        let mut arc_w = *rw.write().unwrap();

        arc_w.push(chan_cron);

    }

    pub fn launch_lastest(&'static self, driver: &'static WebDriver, behavior: &'static Behavior, db: &'static Database) {
        let ReadWriteQueue(rw)  = self;

        let mut arc_w = rw.read().unwrap();

        if let Some(last) = arc_w.last() {
            last.launch(driver, behavior, db);
        }
    }

    pub fn launch_name(&'static self, name: String, driver: &'static WebDriver, behavior: &'static Behavior, db: &'static Database) {
        let ReadWriteQueue(rw)  = self;

        let mut arc_w = rw.read().unwrap();

        for  w in arc_w.into_iter() {
            if w.name == name {
                w.launch(driver, behavior, db);
            }
        }
    }

    pub fn suspend_latest(&'static self, millis: u32) {
        let ReadWriteQueue(rw)  = self;

        let mut arc_w = rw.read().unwrap();

        if let Some(last) = arc_w.last() {
            last.suspend(millis);
        }
    }

    pub fn suspend_name(&'static self, name: String, millis: u32) {
        let ReadWriteQueue(rw)  = self;

        let mut arc_w = rw.read().unwrap();

        for  w in arc_w.into_iter() {
            if w.name == name {
                w.suspend(millis);
            }
        }
    }

    pub fn terminate_latest(&'static self) {
        let ReadWriteQueue(rw)  = self;

        let mut arc_w = rw.read().unwrap();

        if let Some(last) = arc_w.last() {
            last.terminate();
        }
    }

    pub fn terminate_name(&'static self, name: String) {
        let ReadWriteQueue(rw)  = self;

        let mut arc_w = rw.read().unwrap();

        for  w in arc_w.into_iter() {
            if w.name == name {
                w.terminate();
            }
        }
    }
}