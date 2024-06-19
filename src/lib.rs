#![feature(trait_upcasting)]
#![feature(trivial_bounds)]

pub mod config;
pub mod controller;
pub mod di;
pub mod handler;
pub mod mysql;
pub mod util;
pub mod mapper;

use crate::handler::grpc::IGrpcService;
use di::dependency_injector;
use handler::p2p::ISwarm;
use runtime_injector::Svc;
use tokio::try_join;
use util::errors::{die, Result};

pub async fn run() -> Result<()> {
    env_logger::init();
    let injector = dependency_injector()?;
    let grpc_handler: Svc<dyn IGrpcService> = injector.get()?;
    let swarm: Svc<dyn ISwarm> = injector.get()?;

    match try_join!(grpc_handler.start(), swarm.start(),) {
        Err(err) => die(err),
        _ => {}
    };
    Ok(())
}
