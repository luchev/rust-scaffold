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
