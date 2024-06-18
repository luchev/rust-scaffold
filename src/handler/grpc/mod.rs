pub mod init;
pub mod inner;

use crate::config::IConfig;
use crate::util::consts::{GRPC_TIMEOUT, LOCALHOST};
use crate::util::errors::{ErrorKind, Result};
use async_trait::async_trait;
use init::app_grpc::app_service_server::{AppService, AppServiceServer};
use init::app_grpc::PingRequest;
use inner::GrpcInnerHandler;
use log::info;
use runtime_injector::{
    interface, InjectResult, Injector, RequestInfo, Service, ServiceFactory, Svc,
};
use std::borrow::BorrowMut;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio_stream::wrappers::TcpListenerStream;
use tonic::transport::Server;
use tonic::{IntoRequest, Request};

#[async_trait]
pub trait IGrpc: Service {
    async fn start(&self) -> Result<()>;
}

interface! {
    dyn IGrpc = [
        GrpcHandler,
    ]
}

pub struct GrpcProvider;
impl ServiceFactory<()> for GrpcProvider {
    type Result = GrpcHandler;

    fn invoke(
        &mut self,
        injector: &Injector,
        _request_info: &RequestInfo,
    ) -> InjectResult<Self::Result> {
        let port = injector.get::<Svc<dyn IConfig>>()?.grpc().port;
        let log = injector.get::<Svc<dyn IConfig>>()?.log().handler;
        let inner = injector.get::<GrpcInnerHandler>()?;
        Ok(GrpcHandler { inner, port, log })
    }
}

pub struct GrpcHandler {
    inner: GrpcInnerHandler,
    port: u16,
    log: bool,
}

#[async_trait]
impl IGrpc for GrpcHandler {
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
