use std::collections::HashMap;

use super::IMysql;
use crate::util::errors::Result;
use async_trait::async_trait;

pub struct MysqlMock { _pings: HashMap<String, i32> }

impl MysqlMock {
    pub fn new() -> Self {
        Self {
            _pings: HashMap::new(),
        }
    }
}

#[async_trait]
impl IMysql for MysqlMock {
    fn init(&self) {
        println!("mysql mock init");
    }

    async fn ping(&self) -> Result<()> {
        println!("mysql mock ping");
        Ok(())
    }
}
