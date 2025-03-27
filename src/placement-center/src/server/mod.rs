use std::{
  collections::BTreeMap,
  sync::{Arc, RwLock},
};

use grpc::server::start_grpc_server;
use http::server::{start_http_server, HttpServerState};
use log::info;
use tokio::{signal, sync::broadcast};

pub mod grpc;
pub mod http;

pub async fn start_server(kvs: BTreeMap<String, String>, stop_sx: broadcast::Sender<bool>) {
  let state = HttpServerState::new(Arc::new(RwLock::new(kvs)));
 
  // 将 start_grpc_server 运行在一个独立 tokio task 中
  let raw_stop_sx=stop_sx.clone();
  tokio::spawn(async move{
    start_grpc_server(raw_stop_sx).await;
  });

  // 将 start_http_server 运行在一个独立的 tokio task中
  let raw_stop_sx=stop_sx.clone();
  tokio::spawn(async move{
    start_http_server(state, raw_stop_sx).await;
  });

  // 等待进程信号
  awaiting_stop(stop_sx.clone()).await;
}

pub async fn awaiting_stop(stop_send: broadcast::Sender<bool>){
  signal::ctrl_c().await.expect("failed to listen for event");
  match stop_send.send(true) {
      Ok(_)=>{
        info!("{}","When ctrl + c is received, the service starts to stop")
      }
      Err(e)=>{
        panic!("{}",e);
      }
      
  }
}