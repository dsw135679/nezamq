use std::{
  collections::BTreeMap,
  net::SocketAddr,
  sync::{Arc, RwLock},
};

use axum::{Router, routing::get};
use common_base::config::placement_center::placement_center_conf;
use log::info;
use tokio::{select, sync::broadcast};

use super::{index, path_list, v1_path};

pub const ROUTE_ROOT: &str = "/index";
pub const ROUTE_ADD_LEARNER: &str = "/add-learner";
pub const ROUTE_CHANGE_MEMBERSHIP: &str = "/change-membership";
pub const ROUTE_INIT: &str = "/init";
pub const ROUTE_METRICS: &str = "/metrics";
pub const ROUTE_SET: &str = "/set";
pub const ROUTE_GET: &str = "/get";

// 服务状态
#[derive(Debug, Clone)]
pub struct HttpServerState {
  pub kvs: Arc<RwLock<BTreeMap<String, String>>>,
}

impl HttpServerState {
  pub fn new(kvs: Arc<RwLock<BTreeMap<String, String>>>) -> Self {
    Self { kvs }
  }
}

pub async fn start_http_server(state: HttpServerState, stop_sx: broadcast::Sender<bool>) {
  // 读取配置
  let config = placement_center_conf();

  // 组装监听地址和端口
  let ip: SocketAddr = match format!("0.0.0.0:{}", config.network.http_port).parse() {
    Ok(data) => data,
    Err(e) => {
      panic!("{}", e);
    }
  };

  info!(
    "Broker HTTP Server start. port:{}",
    config.network.http_port
  );

  // 构建路由信息
  let app = routes(state);

  let mut stop_rx = stop_sx.subscribe();

  let listener = match tokio::net::TcpListener::bind(ip).await {
    Ok(data) => data,
    Err(e) => {
      panic!("{}", e);
    }
  };

  // 通过 select 来同时监听进程停止信号和 Server 运行
  select! {
      // 监听进程停止信号
      val= stop_rx.recv()=>{
          match val {
              Ok(flag)=>{
                  if flag{
                      info!("HTTP Server stopped successfully");
                  }
              }
              Err(_)=>{}
          }
      }
      // 监听服务
      val=axum::serve(listener,app.clone())=>{
          match val {
              Ok(())=>{}
              Err(e)=>{
                  // HTTP 服务监听失败，直接退出程序
                  panic!("{}",e);
              }
          }
      }

  }
}

// 定义路由
fn routes(state: HttpServerState) -> Router {
  let common=Router::new()
     .route(&v1_path(&path_list(ROUTE_ROOT)),get(index))
    // .route(&v1_path(&path_list(ROUTE_ADD_LEARNER)),get(add_leaderner))
    // .route(&v1_path(&path_list(ROUTE_CHANGE_MEMBERSHIP)),get(change_membership))
    // .route(&v1_path(&path_list(ROUTE_INIT)),get(init))
    // .route(&v1_path(&path_list(ROUTE_METRICS)),get(metrics))
    // .route(&v1_path(&path_list(ROUTE_SET)),get(set))
    // .route(&v1_path(&path_list(ROUTE_GET)),get(kv_get))
    ;

  // 构建路由信息并返回
  let app = Router::new().merge(common);
  return app.with_state(state);
}
