use crate::util::types::{OneReceiver, QueryPingResponse};
use crate::util::{types::CommandToSwarm, errors::Result};
use async_trait::async_trait;
use libp2p::PeerId;
use log::info;
use runtime_injector::{
    interface, InjectResult, Injector, RequestInfo, Service, ServiceFactory, Svc,
};
use tokio::sync::{mpsc, oneshot, Mutex};

#[async_trait]
pub trait IRemoteController: Service {
    async fn ping(&self, peer: PeerId) -> Result<()>;
}

interface! {
    dyn IRemoteController = [
        RemoteController,
    ]
}

pub struct SwarmControllerProvider;
impl ServiceFactory<()> for SwarmControllerProvider {
    type Result = RemoteController;

    fn invoke(
        &mut self,
        injector: &Injector,
        _request_info: &RequestInfo,
    ) -> InjectResult<Self::Result> {
        let commands_to_swarm: Svc<Mutex<mpsc::Sender<CommandToSwarm>>> = injector.get()?;
        Ok(RemoteController { commands_to_swarm })
    }
}


pub struct RemoteController {
    commands_to_swarm: Svc<Mutex<mpsc::Sender<CommandToSwarm>>>,
}

#[async_trait]
impl IRemoteController for RemoteController {
    async fn ping(&self, peer: PeerId) -> Result<()> {
        let (sender, receiver) = oneshot::channel::<OneReceiver<Result<QueryPingResponse>>>();

        self.commands_to_swarm
            .lock()
            .await
            .send(CommandToSwarm::Ping {
                resp: sender,
                peer,
            })
            .await.unwrap();
        let receiving_channel = receiver.await.unwrap();
        let result = receiving_channel.await.unwrap();

        result.map(|_| ())
    }
}
