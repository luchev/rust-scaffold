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
    interface, InjectResult, Injector, Provider, RequestInfo, Service, ServiceInfo, SingletonProvider, Svc
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

// impl ServiceFactory<()> for MysqlProvider {
//     type Result = Svc<dyn IMysql>;

//     fn invoke(
//         &mut self,
//         injector: &Injector,
//         _request_info: &RequestInfo,
//     ) -> InjectResult<Self::Result> {
//         let config = injector.get::<Svc<dyn IConfig>>()?.storage();

//         match config {
//             Storage::Mock => {
//                 info!("using mysql mock");
//                 return Ok(Svc::new(mock::MysqlMock::new()));
//             },
//             _ => todo!(),
//         }
//     }
// }


pub struct MysqlProvider;
impl Provider for MysqlProvider {
    fn provide(
        &mut self,
        injector: &Injector,
        _request_info: &RequestInfo,
    ) -> InjectResult<runtime_injector::DynSvc> {
        let config = injector.get::<Svc<dyn IConfig>>()?.storage();

        match config {
            Storage::Mock => {
                info!("using mysql mock");
                return Ok(Svc::new(MysqlMock::new()));
            }
            Storage::Mysql(config) => {
                info!("using mysql");
                return Ok(Svc::new(Mysql::new(config)));
            }
        }
    }

    fn result(&self) -> ServiceInfo {
        ServiceInfo::of::<dyn IMysql>()
    }
}
