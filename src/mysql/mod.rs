mod mock;
mod mysql;

use crate::{
    config::{IConfig, Storage},
    util::errors::Result,
};
use async_trait::async_trait;
use log::info;
use mock::MysqlMock;
use mysql::Mysql;
use runtime_injector::{
    interface, DynSvc, InjectResult, Injector, Provider, RequestInfo, Service, ServiceInfo, Svc,
};

#[async_trait]
pub trait IMysql: Service {
    fn init(&self);
    async fn ping(&self) -> Result<()>;
}

interface! {
    dyn IMysql = [
        Mysql,
        MysqlMock,
    ]
}

#[derive(Default)]
pub struct MysqlProvider {
    result: Option<DynSvc>,
}

impl Provider for MysqlProvider {
    fn provide(
        &mut self,
        injector: &Injector,
        _request_info: &RequestInfo,
    ) -> InjectResult<runtime_injector::DynSvc> {
        if let Some(ref service) = self.result {
            return Ok(service.clone());
        }

        let config = injector.get::<Svc<dyn IConfig>>()?.storage();
        let result = match config {
            Storage::Mock => {
                info!("using mysql mock");
                Svc::new(MysqlMock::new()) as DynSvc
            }
            Storage::Mysql(config) => {
                info!("using mysql");
                Svc::new(Mysql::new(config)) as DynSvc
            }
        };

        self.result = Some(result.clone());
        Ok(result)
    }

    fn result(&self) -> ServiceInfo {
        ServiceInfo::of::<dyn IMysql>()
    }
}
