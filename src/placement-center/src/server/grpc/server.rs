use common_base::config::placement_center::placement_center_conf;
use log::info;
use protocol::kv_service_server::KvServiceServer;
use tokio::{select, sync::broadcast};
use tonic::transport::Server;

use crate::server::grpc::services_kv::GrpcBrokerServices;

pub async fn start_grpc_server(stop_sx: broadcast::Sender<bool>) {
  let config = placement_center_conf();
  let server = GrpcServer::new(config.network.grpc_port);
  server.start(stop_sx).await;
}

pub struct GrpcServer {
  port: usize,
}

impl GrpcServer {
  pub fn new(port: u32) -> Self {
    return Self { port };
  }

  pub async fn start(&self, stop_sx: broadcast::Sender<bool>) {
    let addr = format!("0.0.0.0:{}", self.port).parse().unwrap();
    info!("Broker Grpc Server start. port:{}", self.port);

    let kv_service_handler = GrpcBrokerServices::new();

    let mut stop_rx = stop_sx.subscribe();

    select! {
        val = stop_rx.recv()=>{
            match val {
                Ok(flag)=>{
                    if flag{
                        info!("HTTP Server stopped successfully");
                    }
                }
                Err(_)=>{}
            }
        },

        val = Server::builder().add_service(KvServiceServer::new(kv_service_handler)).serve(addr)=>{
            match val {
                Ok(())=>{},
                Err(e)=>{
                    panic!("{}",e);
                }
            }
        }
    }
  }
}

#[cfg(test)]
mod tests {
  use protocol::{
    DeleteRequest, ExistsRequest, GetRequest, SetRequest, kv_service_client::KvServiceClient,
  };

  #[tokio::test]
  async fn kv_test() {
    let mut client = KvServiceClient::connect("http://127.0.0.1:8871")
      .await
      .unwrap();

    let key = "mq".to_string();
    let value = "nezama".to_string();
    let request = tonic::Request::new(SetRequest {
      key: key.clone(),
      value: value.clone(),
    });

    let _ = client.set(request).await.unwrap();

    let request = tonic::Request::new(ExistsRequest { key: key.clone() });
    let exist_reply = client.exists(request).await.unwrap().into_inner();
    assert!(exist_reply.flag);

    let request = tonic::Request::new(GetRequest { key: key.clone() });
    let get_reply = client.get(request).await.unwrap().into_inner();
    assert_eq!(get_reply.value, value);

    let request = tonic::Request::new(DeleteRequest { key: key.clone() });
    let _ = client.delete(request).await.unwrap().into_inner();

    let request = tonic::Request::new(ExistsRequest { key: key.clone() });
    let exist_reply = client.exists(request).await.unwrap().into_inner();
    assert!(!exist_reply.flag);
  }
}
