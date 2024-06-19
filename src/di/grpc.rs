use crate::handler::grpc::{
    init::app_grpc::app_service_server::AppService, inner::GrpcHandlerProvider, GrpcServiceProvider,
    IGrpcService,
};
use runtime_injector::{define_module, IntoSingleton};

pub fn module() -> runtime_injector::Module {
    define_module! {
        services = [
            GrpcServiceProvider.singleton(),
            GrpcHandlerProvider.singleton(),
        ],
        interfaces = {
            dyn AppService = [ GrpcHandlerProvider.singleton() ],
            dyn IGrpcService = [ GrpcServiceProvider.singleton() ],
        },
    }
}
