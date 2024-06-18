#![feature(trait_upcasting)]
#![feature(trivial_bounds)]

use config::IConfig;
use di::dependency_injector;
use handler::p2p::ISwarm;
use util::errors::{die, Result};
use log::info;
use runtime_injector::Svc;
use tokio::try_join;
use crate::handler::grpc::IGrpc;


pub mod di;
pub mod config;
pub mod util;
pub mod mysql;
pub mod handler;
pub mod controller;

#[tokio::main]
async fn main() {
    match run().await {
        Ok(()) => info!("shutting down"),
        Err(err) => die(err),
    }
}

async fn run() -> Result<()> {
    env_logger::init();
    let injector = dependency_injector()?;
    let grpc_handler: Svc<dyn IGrpc> = injector.get()?;
    let swarm: Svc<dyn ISwarm> = injector.get()?;

    try_join!(
        grpc_handler.start(),
        swarm.start(),
    )
    .map(|_| ())
}
