mod p2p;

use crate::{
    config::{ConfigProvider, IConfig}, controller::ping::{ILocalController, PingProvider}, handler::grpc::{init::app_grpc::app_service_server::AppService, inner::{GrpcInnerHandler, GrpcInnerProvider}, GrpcProvider, IGrpc}, mysql::{IMysql, MysqlProvider}, util::errors::Result
};
use runtime_injector::{Injector, IntoSingleton, TypedProvider};

pub fn dependency_injector() -> Result<Injector> {
    let mut injector = Injector::builder();
    // injector.add_module(p2p::module());
    injector.provide(ConfigProvider.singleton().with_interface::<dyn IConfig>());
    injector.provide(GrpcProvider.singleton().with_interface::<dyn IGrpc>());
    injector.provide(PingProvider.singleton().with_interface::<dyn ILocalController>());
    injector.provide(MysqlProvider::default());
    injector.add_module(p2p::module());
    injector.provide(GrpcInnerProvider.singleton());

    Ok(injector.build())
}
