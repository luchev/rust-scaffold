mod init;
mod inner;

use std::net::SocketAddr;
use inner::Inner;
use std::time::Duration;
use init::app_grpc::app_service_server::AppServiceServer;
use log::info;
use runtime_injector::{
    interface, InjectResult, Injector, RequestInfo, Service, ServiceFactory, Svc,
};
use tokio::net::TcpListener;
use tonic::transport::Server;
use crate::config::{IConfig, Log};
use crate::controller::ping::IPing;
use crate::util::errors::{ErrorKind, Result};
use crate::mysql::IMysql;
use crate::util::consts::{GRPC_TIMEOUT, LOCALHOST};
use tokio_stream::wrappers::TcpListenerStream;
use async_trait::async_trait;

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
        let controller = injector.get::<Svc<dyn IPing>>()?;

        Ok(GrpcHandler {
            inner: Inner::new(controller, log),
            port,
            log,
        })
    }
}

pub struct GrpcHandler {
    inner: Inner,
    port: u16,
    log: bool,
}

#[async_trait]
impl IGrpc for GrpcHandler {
    async fn start(&self) -> Result<()> {
        let addr = format!("{}:{}", LOCALHOST, self.port)
            .parse::<SocketAddr>()
            .map_err(|e| ErrorKind::Generic(format!("parsing grpc address error: {}", e.to_string())))?;

        let listener = TcpListener::bind(addr)
            .await
            .map_err(|e| ErrorKind::Io(e))?;
        let real_addr = listener
            .local_addr()
            .map_err(|e| ErrorKind::Io(e))?;

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
                AppServiceServer::new(self.inner.clone())
                    .max_decoding_message_size(1024 * 1024 * 1024)
                    .max_encoding_message_size(1024 * 1024 * 1024),
            )
            .serve_with_incoming(TcpListenerStream::new(listener))
            .await?;
        Ok(())
    }
}
