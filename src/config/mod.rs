mod conf;
mod types;

use crate::{
    util::consts::{BASE_CONFIG, CONFIG_DIR, CONFIG_PREFIX},
    util::errors::{Error, ErrorKind},
};
use conf::Config;
use config::{Environment, File};
use log::info;
use runtime_injector::{
    interface, InjectError, InjectResult, Injector, RequestInfo, Service, ServiceFactory,
    ServiceInfo,
};
use std::{env, path::Path};
pub use types::*;

pub trait IConfig: Service {
    fn grpc(&self) -> Grpc;
    fn log(&self) -> Log;
    fn storage(&self) -> Storage;
}

interface! {
    dyn IConfig = [
        Config,
    ]
}

pub struct ConfigProvider;
impl ServiceFactory<()> for ConfigProvider {
    type Result = Config;

    fn invoke(
        &mut self,
        _injector: &Injector,
        _request_info: &RequestInfo,
    ) -> InjectResult<Self::Result> {
        let env_conf = env::var("ENV").unwrap_or_else(|_| "dev".into());

        if !Path::new(&format!("{}/{}.yaml", CONFIG_DIR, env_conf)).exists() {
            info!("creating new config file for env: {}", env_conf);
            let conf = Config::default();
            let serialized = serde_yaml::to_string(&conf).unwrap_or_default();
            std::fs::write(format!("{}/{}.yaml", CONFIG_DIR, env_conf), serialized)
                .unwrap_or_default();
        }

        let mut builder = config::Config::builder();
        if Path::new(BASE_CONFIG).exists() {
            builder = builder.add_source(File::with_name(BASE_CONFIG));
        }

        builder
            .add_source(File::with_name(&format!("{}/{}", CONFIG_DIR, env_conf)).required(false))
            .add_source(
                Environment::with_prefix(CONFIG_PREFIX)
                    .try_parsing(true)
                    .separator("_")
                    .list_separator(":"),
            )
            .build()
            .map_err(|err| InjectError::ActivationFailed {
                service_info: ServiceInfo::of::<Config>(),
                inner: Box::<Error>::new(ErrorKind::ConfigErr(err).into()),
            })?
            .try_deserialize()
            .map_err(|err| InjectError::ActivationFailed {
                service_info: ServiceInfo::of::<Config>(),
                inner: Box::<Error>::new(ErrorKind::ConfigErr(err).into()),
            })
    }
}
