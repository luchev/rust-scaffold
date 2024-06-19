use super::IMysql;
use crate::{config, util::errors::Result};
use async_trait::async_trait;

pub struct Mysql {
    pub config: config::Mysql,
}

impl Mysql {
    pub fn new(config: config::Mysql) -> Self {
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
        // Err("ping error".into())
        Ok(())
    }
}
