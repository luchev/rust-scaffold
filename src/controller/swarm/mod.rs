use crate::util::types::{Bytes, OneReceiver, QueryGetResponse};
use crate::util::{types::CommandToSwarm, errors::Result};
use async_trait::async_trait;
use runtime_injector::{
    interface, InjectResult, Injector, RequestInfo, Service, ServiceFactory, Svc,
};
use tokio::sync::{mpsc, oneshot, Mutex};
use uuid::Uuid;

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

#[async_trait]
pub trait IRemoteController: Service {
    async fn put(&self, key: Uuid, value: Bytes) -> Result<()>;
    async fn get(&self, key: Uuid) -> Result<QueryGetResponse>;
}

pub struct RemoteController {
    commands_to_swarm: Svc<Mutex<mpsc::Sender<CommandToSwarm>>>,
}

#[async_trait]
impl IRemoteController for RemoteController {
    async fn put(&self, key: Uuid, value: Bytes) -> Result<()> {
        let (sender, receiver) = oneshot::channel::<OneReceiver<Result<()>>>();

        self.commands_to_swarm
            .lock()
            .await
            .send(CommandToSwarm::Put {
                key,
                value,
                resp: sender,
            })
            .await.unwrap();
        let receiving_channel = receiver.await.unwrap();
        let result = receiving_channel.await.unwrap();

        result
    }

    async fn get(&self, key: Uuid) -> Result<QueryGetResponse> {
        let (sender, receiver) = oneshot::channel::<OneReceiver<Result<QueryGetResponse>>>();
        self.commands_to_swarm
            .lock()
            .await
            .send(CommandToSwarm::Get { key, resp: sender })
            .await.unwrap();
        let receiving_channel = receiver.await.unwrap();
        let result = receiving_channel.await.unwrap();
        result
    }
}
