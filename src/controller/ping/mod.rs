use async_trait::async_trait;
use runtime_injector::{interface, InjectResult, Injector, RequestInfo, Service, ServiceFactory, Svc};

use crate::{mysql::IMysql, util::errors::Result};

#[async_trait]
pub trait IPing: Service {
    async fn ping(&self) -> Result<()>;
}

interface! {
    dyn IPing = [
        Ping,
    ]
}

pub struct PingProvider;
impl ServiceFactory<()> for PingProvider {
    type Result = Ping;

    fn invoke(
        &mut self,
        injector: &Injector,
        _request_info: &RequestInfo,
    ) -> InjectResult<Self::Result> {
        let mysql = injector.get::<Svc<dyn IMysql>>()?;
        let mysql2 = injector.get::<Svc<dyn IMysql>>()?;

        Ok(Ping { mysql })
    }
}

pub struct Ping {
    mysql: Svc<dyn IMysql>,
}

#[async_trait]
impl IPing for Ping {
    async fn ping(&self) -> Result<()> {
        self.mysql.ping().await
    }
}
