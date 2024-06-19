use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Mysql {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub db_name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Storage {
    Mysql(Mysql),
    Mock,
}

impl Default for Storage {
    fn default() -> Self {
        Self::Mock
    }
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Grpc {
    pub port: u16,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Log {
    pub handler: bool,
    pub controller: bool,
}
