use crate::{
    config::{ConfigProvider, IConfig}, controller::ping::{IPing, PingProvider}, handler::grpc::{GrpcProvider, IGrpc}, mysql::{IMysql, MysqlProvider}, util::errors::Result
};
use runtime_injector::{Injector, IntoSingleton, SingletonProvider, TypedProvider, WithCondition};

pub fn dependency_injector() -> Result<Injector> {
    let mut injector = Injector::builder();
    // injector.add_module(p2p::module());
    injector.provide(ConfigProvider.singleton().with_interface::<dyn IConfig>());
    // injector.provide(MysqlProvider.singleton().with_interface::<dyn IMysql>());
    injector.provide(GrpcProvider.singleton().with_interface::<dyn IGrpc>());
    injector.provide(PingProvider.singleton().with_interface::<dyn IPing>());
    injector.provide(MysqlProvider);

    Ok(injector.build())
}
