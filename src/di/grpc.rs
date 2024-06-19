use crate::handler::grpc::{
    init::app_grpc::app_service_server::AppService, inner::GrpcInnerProvider, GrpcProvider,
    IGrpcHandler,
};
use runtime_injector::{define_module, IntoSingleton};

pub fn module() -> runtime_injector::Module {
    define_module! {
        services = [
            GrpcProvider.singleton(),
            GrpcInnerProvider.singleton(),
        ],
        interfaces = {
            dyn AppService = [ GrpcInnerProvider.singleton() ],
            dyn IGrpcHandler = [ GrpcProvider.singleton() ],
        },
    }
}
