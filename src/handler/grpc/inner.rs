use log::{error, info};
use runtime_injector::Svc;
use tonic::{Request, Response, Status};
use crate::{controller::ping::IPing, mysql::IMysql};
use async_trait::async_trait;

use super::init::app_grpc::{app_service_server::AppService, PingRequest, PingResponse};

#[derive(Clone)]
pub struct Inner {
    controller: Svc<dyn IPing>,
    log: bool,
}

impl Inner {
    pub fn new(controller: Svc<dyn IPing>, log: bool) -> Self {
        Self { controller, log }
    }
}

#[async_trait]
impl AppService for Inner {
    async fn ping(
        &self,
        request: Request<PingRequest>,
    ) -> std::result::Result<Response<PingResponse>, Status> {
        let _request = request.into_inner();

        if self.log {
            info!("received ping request");
        }

        match self.controller.ping().await {
            Ok(_) => {
                if self.log {
                    info!("ping success");
                }
                Ok(Response::new(PingResponse { }))
            }
            Err(e) => {
                error!("ping failed: {}", e);
                Err(Status::internal("ping failed"))
            }

        }
    }
}
