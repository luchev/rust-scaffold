use crate::util::{errors::Result as Res, types::{CommandToSwarm, QueryPingResponse}};
use async_trait::async_trait;
use libp2p::{
    futures::StreamExt,
    identity::Keypair,
    noise,
    request_response::{self, ProtocolSupport},
    swarm::NetworkBehaviour,
    tcp, yamux, StreamProtocol, SwarmBuilder,
};
use log::info;
use runtime_injector::{
    interface, InjectError, InjectResult, Injector, RequestInfo, Service, ServiceFactory,
    ServiceInfo, Svc,
};
use serde::{Deserialize, Serialize};
use tokio::{
    select,
    sync::{mpsc::Receiver, Mutex, oneshot},
};

#[async_trait]
pub trait ISwarm: Service {
    async fn start(&self) -> Res<()>;
}

interface! {
    dyn ISwarm = [
        Swarm,
    ]
}

pub struct Swarm {
    swarm: Mutex<libp2p::Swarm<Behaviour>>,
    commands_from_controller: Svc<Mutex<Receiver<CommandToSwarm>>>,
}

pub struct SwarmProvider;
impl ServiceFactory<()> for SwarmProvider {
    type Result = Swarm;

    fn invoke(
        &mut self,
        injector: &Injector,
        _request_info: &RequestInfo,
    ) -> InjectResult<Self::Result> {
        let commands_from_controller: Svc<Mutex<Receiver<CommandToSwarm>>> = injector.get()?;

        let mut swarm = SwarmBuilder::with_existing_identity(Keypair::generate_ed25519())
            .with_tokio()
            .with_tcp(
                tcp::Config::default(),
                noise::Config::new,
                yamux::Config::default,
            )
            .map_err(|e| InjectError::ActivationFailed {
                service_info: ServiceInfo::of::<Swarm>(),
                inner: Box::new(e),
            })?
            .with_behaviour(|_key| {
                Ok(Behaviour {
                    req_res: request_response::cbor::Behaviour::new(
                        [(
                            StreamProtocol::new("/verification/1.0.0"),
                            ProtocolSupport::Full,
                        )],
                        request_response::Config::default(),
                    ),
                })
            })
            .map_err(|e| InjectError::ActivationFailed {
                service_info: ServiceInfo::of::<Swarm>(),
                inner: Box::new(e),
            })?
            .build();

        swarm
            .listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap())
            .map_err(|e| InjectError::ActivationFailed {
                service_info: ServiceInfo::of::<Swarm>(),
                inner: Box::new(e),
            })?;

        Ok(Swarm {
            swarm: Mutex::new(swarm),
            commands_from_controller,
        })
    }
}

#[derive(NetworkBehaviour)]
pub struct Behaviour {
    req_res: request_response::cbor::Behaviour<PingRequest, PingResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PingRequest {}

#[derive(Debug, Serialize, Deserialize)]
pub struct PingResponse {
    success: bool,
}

#[async_trait]
impl ISwarm for Swarm {
    async fn start(&self) -> Res<()> {
        let mut swarm = self.swarm.lock().await;
        let mut receiver = self.commands_from_controller.lock().await;
        loop {
            select! {
                instruction = receiver.recv() => {
                    info!("Received instruction from controller");
                    match instruction {
                        Some(CommandToSwarm::Ping { resp, peer: _ }) => {
                            info!("Received ping instruction");
                            let (sender, receiver) = oneshot::channel::<Res<QueryPingResponse>>();
                            resp.send(receiver).unwrap();
                            sender.send(Ok(QueryPingResponse{})).unwrap();
                        },
                        _ => {},
                    };
                },
                _event = swarm.select_next_some() => {
                    info!("Received event from swarm");
                }
            }
        }
    }
}
