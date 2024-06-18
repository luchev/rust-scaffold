use config::IConfig;
use di::dependency_injector;
use util::errors::{die, Result};
use log::info;
use runtime_injector::Svc;
use tokio::try_join;
use crate::handler::grpc::IGrpc;

mod di;
mod config;
mod util;
mod mysql;
mod handler;
mod controller;

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
    let _settings: Svc<dyn IConfig> = injector.get()?;

    try_join!(
        grpc_handler.start(),
    )
    .map(|_| ())
}
