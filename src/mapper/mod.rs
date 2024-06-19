use crate::util::errors::Result;
use libp2p::PeerId;
use runtime_injector::{interface, InjectResult, Injector, RequestInfo, Service, ServiceFactory};
use std::str::FromStr;

pub trait IMapper: Service {
    fn peer_id_from_string(&self, peer_id: String) -> Result<libp2p::PeerId>;
}

interface! {
    dyn IMapper = [
        Mapper,
    ]
}

pub struct MapperProvider;
impl ServiceFactory<()> for MapperProvider {
    type Result = Mapper;

    fn invoke(
        &mut self,
        _injector: &Injector,
        _request_info: &RequestInfo,
    ) -> InjectResult<Self::Result> {
        Ok(Mapper {})
    }
}

pub struct Mapper {}

impl IMapper for Mapper {
    fn peer_id_from_string(&self, peer_id: String) -> Result<PeerId> {
        Ok(PeerId::from_str(peer_id.as_str())?)
    }
}
