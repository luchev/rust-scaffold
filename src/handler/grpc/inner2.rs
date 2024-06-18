use crate::controller::{ping::ILocalController, swarm::IRemoteController};
use async_trait::async_trait;
use log::{error, info};
use runtime_injector::Svc;
use tonic::{Request, Response, Status};
use super::init::app_grpc::{app_service_server::AppService, PingRequest, PingResponse};

#[derive(Clone)]
pub struct GrpcHandlerService {
    local_controller: Svc<dyn ILocalController>,
    remote_controller: Svc<dyn IRemoteController>,
    log: bool,
}

impl GrpcHandlerService {
    pub fn new(
        local_controller: Svc<dyn ILocalController>,
        remote_controller: Svc<dyn IRemoteController>,
        log: bool,
    ) -> Self {
        Self {
            local_controller,
            remote_controller,
            log,
        }
    }
}

#[async_trait]
impl AppService for GrpcHandlerService {
    async fn ping(
        &self,
        request: Request<PingRequest>,
    ) -> std::result::Result<Response<PingResponse>, Status> {
        let _request = request.into_inner();

        if self.log {
            info!("received ping request");
        }

        match self.local_controller.ping().await {
            Ok(_) => {
                if self.log {
                    info!("ping success");
                }
                Ok(Response::new(PingResponse {}))
            }
            Err(e) => {
                error!("ping failed: {}", e);
                Err(Status::internal("ping failed"))
            }
        }
    }
}
