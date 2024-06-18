use serde::{Deserialize, Serialize};
use super::{Grpc, IConfig, Log, Mysql, Storage};

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Config {
    pub grpc: Grpc,
    pub log: Log,
    pub storage: Storage,
}

impl IConfig for Config {
    fn grpc(&self) -> Grpc {
        self.grpc.clone()
    }

    fn log(&self) -> super::Log {
        self.log.clone()
    }

    fn storage(&self) -> super::Storage {
        self.storage.clone()
    }
}

impl Config {
    pub fn default() -> Self {
        Self {
            storage: Storage::Mysql (Mysql{
                username: "mysql".into(),
                password: "mysql".into(),
                host: "localhost".into(),
                port: 3306,
                db_name: "db".into(),
            }),
            grpc: Grpc {
                port: 2000,
            },
            log: Log{
                handler: false,
                controller: false,
            },
        }
    }
}
