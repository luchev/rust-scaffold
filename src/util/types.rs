use super::errors::Result;
use libp2p::PeerId;
use tokio::sync::oneshot;

pub type Bytes = Vec<u8>;
pub type Responder<T> = oneshot::Sender<T>;
pub type OneSender<T> = oneshot::Sender<T>;
pub type OneReceiver<T> = oneshot::Receiver<T>;

#[derive(Debug)]
pub enum CommandToSwarm {
    Ping {
        peer: PeerId,
        resp: Responder<OneReceiver<Result<QueryPingResponse>>>,
    }
}

#[derive(Debug)]
pub struct QueryPingResponse {
}
