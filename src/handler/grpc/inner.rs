use super::init::app_grpc::{app_service_server::AppService, PingRemoteRequest, PingRemoteResponse, PingRequest, PingResponse};
use crate::{
    config::IConfig,
    controller::{local::ILocalController, remote::IRemoteController}, mapper::IMapper,
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

impl InjectorRequest for GrpcInnerHandler {
    fn request(injector: &Injector, _info: &RequestInfo) -> InjectResult<Self> {
        let log = injector.get::<Svc<dyn IConfig>>()?.log().handler;
        let local_controller = injector.get::<Svc<dyn ILocalController>>()?;
        let remote_controller = injector.get::<Svc<dyn IRemoteController>>()?;
        let mapper = injector.get::<Svc<dyn IMapper>>()?;
        Ok(GrpcInnerHandler {
            local_controller,
            remote_controller,
            mapper,
            log,
        })
    }
}

#[derive(Clone)]
pub struct GrpcInnerHandler {
    local_controller: Svc<dyn ILocalController>,
    remote_controller: Svc<dyn IRemoteController>,
    mapper: Svc<dyn IMapper>,
    log: bool,
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

    async fn ping_remote(
        &self,
        request: Request<PingRemoteRequest>,
    ) -> std::result::Result<Response<PingRemoteResponse>, Status> {
        let request = request.into_inner();

        if self.log {
            info!("received remote ping request");
        }

        let peer_id = self.mapper.peer_id_from_string(request.peer).unwrap();

        match self.remote_controller.ping(peer_id).await {
            Ok(_) => {
                if self.log {
                    info!("remote ping success");
                }
                Ok(Response::new(PingRemoteResponse {}))
            }
            Err(e) => {
                error!("remote ping failed: {}", e);
                Err(Status::internal("remote ping failed"))
            }
        }
    }
}
