use std::{sync::Arc, marker::PhantomData};

use crate::config::Config;
use crate::cronueue::CronueueAction;
use crate::proxy::Proxy;
use thirtyfour::prelude::WebDriverResult;
use tokio;
use crate::read_write_queue::{ReadWriteQueue, CronChannel};
use std::sync::{RwLock, Arc, Mutex};

pub struct Bot<'a> {
    name: String,
    queue: ReadWriteQueue<'a>,
    config: Config,
    proxy: Proxy,

}

impl<'a> Bot<'a> {
    pub fn new(name_raw: String, config: Config, proxy: Proxy) -> Self {
        let name_clone = name_raw.clone();
        let mut name = name_clone;
        name.push_str("-bot");

        let _phantom = PhantomData;


        Bot {
            queue,
            config,
            proxy,
            _phantom
        }
    }


}
