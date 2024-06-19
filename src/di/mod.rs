mod grpc;
mod p2p;

use crate::{
    config::{ConfigProvider, IConfig},
    controller::local::{ILocalController, PingProvider},
    mapper::{IMapper, MapperProvider},
    mysql:: MysqlProvider,
    util::errors::Result,
};
use runtime_injector::{Injector, IntoSingleton, TypedProvider};

pub fn dependency_injector() -> Result<Injector> {
    let mut injector = Injector::builder();

    injector.provide(ConfigProvider.singleton().with_interface::<dyn IConfig>());
    injector.provide(
        PingProvider
            .singleton()
            .with_interface::<dyn ILocalController>(),
    );
    injector.provide(MysqlProvider::default());
    injector.provide(MapperProvider.singleton().with_interface::<dyn IMapper>());
    injector.add_module(p2p::module());
    injector.add_module(grpc::module());

    Ok(injector.build())
}
