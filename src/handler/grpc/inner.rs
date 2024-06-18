use super::init::app_grpc::{app_service_server::AppService, PingRequest, PingResponse};
use crate::{
    config::IConfig,
    controller::{ping::ILocalController, swarm::IRemoteController},
};
use async_trait::async_trait;
use log::{error, info};
use runtime_injector::{
    interface, InjectResult, Injector, Request as InjectorRequest, RequestInfo, ServiceFactory, Svc,
};
use tonic::{Request, Response, Status};

interface! {
    dyn AppService = [
        GrpcInnerHandler,
    ]
}

#[derive(Clone)]
pub struct GrpcInnerHandler {
    local_controller: Svc<dyn ILocalController>,
    remote_controller: Svc<dyn IRemoteController>,
    log: bool,
}

impl InjectorRequest for GrpcInnerHandler {
    fn request(injector: &Injector, _info: &RequestInfo) -> InjectResult<Self> {
        let log = injector.get::<Svc<dyn IConfig>>()?.log().handler;
        let local_controller = injector.get::<Svc<dyn ILocalController>>()?;
        let remote_controller = injector.get::<Svc<dyn IRemoteController>>()?;
        Ok(GrpcInnerHandler {
            local_controller,
            remote_controller,
            log,
        })
    }
}

pub struct GrpcInnerProvider;
impl ServiceFactory<()> for GrpcInnerProvider {
    type Result = GrpcInnerHandler;

    fn invoke(
        &mut self,
        injector: &Injector,
        _request_info: &RequestInfo,
    ) -> InjectResult<Self::Result> {
        GrpcInnerHandler::request(injector, _request_info)
    }
}

#[async_trait]
impl AppService for GrpcInnerHandler {
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
