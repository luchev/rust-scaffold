use std::collections::HashMap;

use crate::util::errors::Result;
use super::IMysql;
use async_trait::async_trait;

pub struct MysqlMock {
    pings: HashMap<String, i32>,
}

impl MysqlMock {
    pub fn new() -> Self {
        Self { pings: HashMap::new() }
    }
}

#[async_trait]
impl IMysql for MysqlMock {
    fn init(&self) {
        println!("mysql mock init");
    }

    async fn ping(&self) -> Result<()> {
        println!("mysql mock ping");
        // Err("ping error".into())
        Ok(())
    }
}
