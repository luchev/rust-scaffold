use std::sync::atomic::{AtomicU32, Ordering};

use crate::{config, util::errors::Result};
use super::IMysql;
use async_trait::async_trait;
use log::info;

static INSTANCE_COUNT: AtomicU32 = AtomicU32::new(0);

pub struct Mysql {
    pub config: config::Mysql,
}

impl Mysql {
    pub fn new(config: config::Mysql) -> Self {
        info!("mysql instance count: {}", INSTANCE_COUNT.fetch_add(1, Ordering::SeqCst));

        Self { config }
    }
}

#[async_trait]
impl IMysql for Mysql {
    fn init(&self) {
        println!("mysql init");
    }

    async fn ping(&self) -> Result<()> {
        println!("mysql ping");
        Err("ping error".into())
        // Ok(())
    }
}
