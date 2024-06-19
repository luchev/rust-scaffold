use async_trait::async_trait;
use runtime_injector::{interface, InjectResult, Injector, RequestInfo, Service, ServiceFactory, Svc};

use crate::{mysql::IMysql, util::errors::Result};

#[async_trait]
pub trait ILocalController: Service {
    async fn ping(&self) -> Result<()>;
}

interface! {
    dyn ILocalController = [
        LocalController,
    ]
}

pub struct PingProvider;
impl ServiceFactory<()> for PingProvider {
    type Result = LocalController;

    fn invoke(
        &mut self,
        injector: &Injector,
        _request_info: &RequestInfo,
    ) -> InjectResult<Self::Result> {
        let mysql = injector.get::<Svc<dyn IMysql>>()?;

        Ok(LocalController { mysql })
    }
}

pub struct LocalController {
    mysql: Svc<dyn IMysql>,
}

#[async_trait]
impl ILocalController for LocalController {
    async fn ping(&self) -> Result<()> {
        self.mysql.ping().await
    }
}
