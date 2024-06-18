use tokio::sync::oneshot;
use uuid::Uuid;

use super::errors::Result;

pub type Bytes = Vec<u8>;
pub type Responder<T> = oneshot::Sender<T>;
pub type OneSender<T> = oneshot::Sender<T>;
pub type OneReceiver<T> = oneshot::Receiver<T>;

#[derive(Debug)]
pub enum CommandToSwarm {
    Get {
        key: Uuid,
        resp: Responder<OneReceiver<Result<QueryGetResponse>>>,
    },
    Put {
        key: Uuid,
        value: Bytes,
        resp: Responder<OneReceiver<Result<()>>>,
    },
}

#[derive(Debug)]
pub struct QueryGetResponse {
    pub data: Bytes,
}
