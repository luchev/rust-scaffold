pub mod init;
pub mod inner;

use crate::{
    config::IConfig,
    util::{
        consts::{GRPC_TIMEOUT, LOCALHOST},
        errors::{ErrorKind, Result},
    },
};
use async_trait::async_trait;
use init::app_grpc::app_service_server::AppServiceServer;
use inner::GrpcHandler;
use log::info;
use runtime_injector::{
    interface, InjectResult, Injector, RequestInfo, Service, ServiceFactory, Svc,
};
use std::{net::SocketAddr, time::Duration};
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;

#[async_trait]
pub trait IGrpcService: Service {
    async fn start(&self) -> Result<()>;
}

interface! {
    dyn IGrpcService = [
        GrpcService,
    ]
}

pub struct GrpcServiceProvider;
impl ServiceFactory<()> for GrpcServiceProvider {
    type Result = GrpcService;

    fn invoke(
        &mut self,
        injector: &Injector,
        _request_info: &RequestInfo,
    ) -> InjectResult<Self::Result> {
        let port = injector.get::<Svc<dyn IConfig>>()?.grpc().port;
        let log = injector.get::<Svc<dyn IConfig>>()?.log().handler;
        let inner = injector.get::<GrpcHandler>()?;
        Ok(GrpcService { inner, port, log })
    }
}

pub struct GrpcService {
    inner: GrpcHandler,
    port: u16,
    log: bool,
}

#[async_trait]
impl IGrpcService for GrpcService {
    async fn start(&self) -> Result<()> {
        let addr = format!("{}:{}", LOCALHOST, self.port)
            .parse::<SocketAddr>()
            .map_err(|e| {
                ErrorKind::Generic(format!("parsing grpc address error: {}", e.to_string()))
            })?;

        let listener = TcpListener::bind(addr)
            .await
            .map_err(|e| ErrorKind::Io(e))?;
        let real_addr = listener.local_addr().map_err(|e| ErrorKind::Io(e))?;

        if self.log {
            info!("grpc listening on {}", real_addr);
        }

        let middleware = tower::ServiceBuilder::new()
            .timeout(Duration::from_secs(GRPC_TIMEOUT))
            .layer(tonic::service::interceptor(Ok))
            .into_inner();

        Server::builder()
            .layer(middleware)
            .add_service(
                AppServiceServer::new(self.inner.to_owned())
                    .max_decoding_message_size(1024 * 1024 * 1024)
                    .max_encoding_message_size(1024 * 1024 * 1024),
            )
            .serve_with_incoming(TcpListenerStream::new(listener))
            .await?;
        Ok(())
    }
}
