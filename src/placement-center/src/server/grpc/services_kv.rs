use dashmap::DashMap;

use protocol::{
  kv_service_server::KvService, CommonReply, DeleteRequest, ExistsReply, ExistsRequest, GetReply,
  GetRequest, SetRequest,
};
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
pub struct GrpcBrokerServices {
  data: DashMap<String, String>,
}

impl GrpcBrokerServices {
  pub fn new() -> Self {
    return GrpcBrokerServices {
      data: DashMap::with_capacity(8),
    };
  }
}

#[tonic::async_trait]
impl KvService for GrpcBrokerServices {
  async fn set(
    &self,
    request: Request<SetRequest>,
  ) -> Result<Response<CommonReply>, Status> {
    let req = request.into_inner();
    self.data.insert(req.key, req.value);
    return Ok(Response::new(CommonReply::default()));
  }

  async fn delete(&self, request: Request<DeleteRequest>) -> Result<Response<CommonReply>, Status> {
    let req = request.into_inner();
    self.data.remove(&req.key);
    return Ok(Response::new(CommonReply::default()));
  }

  async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetReply>, Status> {
    let req = request.into_inner();
    if let Some(data) = self.data.get(&req.key) {
      return Ok(Response::new(GetReply {
        value: data.value().clone(),
      }));
    }
    return Ok(Response::new(GetReply::default()));
  }

  async fn exists(&self, request: Request<ExistsRequest>) -> Result<Response<ExistsReply>, Status> {
    let req = request.into_inner();
    return Ok(Response::new(ExistsReply {
      flag: self.data.contains_key(&req.key),
    }));
  }
}
