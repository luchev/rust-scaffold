use app::{
    di::dependency_injector,
    handler::grpc::{
        init::app_grpc::{
            app_service_client::AppServiceClient, app_service_server::AppServiceServer, PingRemoteRequest, PingRemoteResponse, PingRequest, PingResponse
        },
        inner::GrpcInnerHandler,
    },
};
use runtime_injector::Injector;
use std::{env, sync::Arc};
use std::{fs::remove_file, future::Future};
use tempfile::NamedTempFile;
use tokio::net::{UnixListener, UnixStream};
use tokio_stream::wrappers::UnixListenerStream;
use tonic::transport::{Channel, Endpoint, Server, Uri};
use tower::service_fn;

async fn server_and_client_stub(
    injector: Injector,
) -> (impl Future<Output = ()>, AppServiceClient<Channel>) {
    let socket = NamedTempFile::new().unwrap();
    let socket = Arc::new(socket.into_temp_path());
    remove_file(&*socket).unwrap();

    let uds = UnixListener::bind(&*socket).unwrap();
    let stream = UnixListenerStream::new(uds);

    let grpc_inner = injector.get::<GrpcInnerHandler>().unwrap();

    let serve_future = async move {
        let result = Server::builder()
            .add_service(AppServiceServer::new(grpc_inner))
            .serve_with_incoming(stream)
            .await;
        assert!(result.is_ok());
    };

    let socket = Arc::clone(&socket);
    let channel = Endpoint::try_from("http://any.url")
        .unwrap()
        .connect_with_connector(service_fn(move |_: Uri| {
            let socket = Arc::clone(&socket);
            async move { UnixStream::connect(&*socket).await }
        }))
        .await
        .unwrap();

    let client = AppServiceClient::new(channel);

    (serve_future, client)
}

// The actual test is here
#[tokio::test]
async fn test_local_ping() {
    env::set_var("ENV", "test");

    let di = dependency_injector().unwrap();
    let (serve_future, mut client) = server_and_client_stub(di).await;

    let request_future = async {
        let response = client.ping(PingRequest {}).await.unwrap().into_inner();
        assert_eq!(response, PingResponse {});
    };

    tokio::select! {
        _ = serve_future => panic!("server returned first"),
        _ = request_future => (),
    }
}

// #[tokio::test]
// async fn test_remote_ping() {
//     let di = dependency_injector().unwrap();
//     let (serve_future, mut client) = server_and_client_stub(di).await;

//     let request_future = async {
//         let response = client
//             .ping_remote(PingRemoteRequest {
//                 peer: "QmYyQSo1c1Ym7orWxLYvCrM2EmxFTANf8wXmmE7DWjhx5N".to_string(),
//             })
//             .await
//             .unwrap()
//             .into_inner();
//         assert_eq!(response, PingRemoteResponse {});
//     };

//     tokio::select! {
//         _ = serve_future => panic!("server returned first"),
//         _ = request_future => (),
//     }
// }
