use toml::Table;

use super::{
  common::Log,
  placement_center::{Heartbeat, Network, Node, Rocksdb, System},
};

pub fn default_cluster_name() -> String {
  "placement-center".to_string()
}

pub fn default_node() -> Node {
  Node {
    node_id: default_node_id(),
    nodes: default_nodes(),
  }
}

pub fn default_node_id() -> u64 {
  1
}

pub fn default_nodes() -> Table {
  let mut nodes = Table::new();
  nodes.insert(
    default_node_id().to_string(),
    toml::Value::String(format!("127.0.0.1:{}", default_grpc_port())),
  );
  nodes
}

pub fn default_network() -> Network {
  Network {
    local_id: default_local_ip(),
    grpc_port: default_grpc_port(),
    http_port: default_http_port(),
  }
}

pub fn default_local_ip() -> String {
  "127.0.0.1".to_string()
}

pub fn default_grpc_port() -> u32 {
  1228
}

pub fn default_http_port() -> u32 {
  1227
}

pub fn default_system() -> System {
  System {
    runtime_work_threads: default_runtime_work_threads(),
  }
}

pub fn default_runtime_work_threads() -> usize {
  100
}

pub fn default_data_path() -> String {
  "./nezamq-data/placement-center/data".to_string()
}

pub fn default_log() -> Log {
  Log {
    log_config: "./logs/placement-center".to_string(),
    log_path: "./config/log4rs.yaml".to_string(),
  }
}

pub fn default_max_open_files() -> Option<i32> {
  Some(10000_i32)
}

pub fn default_rocksdb() -> Rocksdb {
  Rocksdb {
    data_path: default_data_path(),
    max_open_files: default_max_open_files(),
  }
}

pub fn default_heartbeat() -> Heartbeat {
  Heartbeat {
    hearbeat_timeout_ms: default_hearbeat_timeout_ms(),
    heartbeat_check_time_ms: default_heartbeat_check_time_ms(),
  }
}

pub fn default_hearbeat_timeout_ms() -> u64 {
  3000
}

pub fn default_heartbeat_check_time_ms() -> u64 {
  1000
}
