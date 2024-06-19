use runtime_injector::{constant, define_module, IntoSingleton};
use tokio::sync::{mpsc::channel, Mutex};
use crate::{
    controller::remote::{IRemoteController, SwarmControllerProvider}, handler::p2p::{ISwarm, SwarmProvider},
    util::types::CommandToSwarm,
};

pub fn module() -> runtime_injector::Module {
    let (sender_from_controller, receiver_in_swarm) = channel::<CommandToSwarm>(5);
    define_module! {
        services = [
            SwarmControllerProvider.singleton(),
            SwarmProvider.singleton(),
            constant(Mutex::new(sender_from_controller)),
            constant(Mutex::new(receiver_in_swarm)),
        ],
        interfaces = {
            dyn IRemoteController = [ SwarmControllerProvider.singleton() ],
            dyn ISwarm = [ SwarmProvider.singleton() ],
        },
    }
}
