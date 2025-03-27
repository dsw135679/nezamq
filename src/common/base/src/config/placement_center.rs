use std::sync::OnceLock;

use crate::config::default_placement_center::*;
use serde::{Deserialize, Serialize};
use toml::Table;

use crate::tools::read_file;

use super::common::Log;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct PlacementCenterConfig {
  #[serde(default = "default_cluster_name")]
  pub cluster_name: String,
  #[serde(default = "default_node")]
  pub node: Node,
  #[serde(default = "default_network")]
  pub network: Network,
  #[serde(default = "default_system")]
  pub system: System,
  #[serde(default = "default_heartbeat")]
  pub heartbeat: Heartbeat,
  #[serde(default = "default_rocksdb")]
  pub rocksdb: Rocksdb,
  #[serde(default = "default_log")]
  pub log: Log,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Node {
  #[serde(default = "default_node_id")]
  pub node_id: u64,
  #[serde(default = "default_nodes")]
  pub nodes: Table,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
pub struct Network {
  #[serde(default = "default_local_ip")]
  pub local_id: String,
  #[serde(default = "default_grpc_port")]
  pub grpc_port: u32,
  #[serde(default = "default_http_port")]
  pub http_port: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
pub struct System {
  #[serde(default = "default_runtime_work_threads")]
  pub runtime_work_threads: usize,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
pub struct Heartbeat {
  #[serde(default = "default_hearbeat_timeout_ms")]
  pub hearbeat_timeout_ms: u64,
  #[serde(default = "default_heartbeat_check_time_ms")]
  pub heartbeat_check_time_ms: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
pub struct Rocksdb {
  #[serde(default = "default_data_path")]
  pub data_path: String,
  #[serde(default = "default_max_open_files")]
  pub max_open_files: Option<i32>,
}

static PLACEMENT_CENTER_CONF: OnceLock<PlacementCenterConfig> = OnceLock::new();

pub fn init_placement_center_conf_by_path(config_path: &String) -> &'static PlacementCenterConfig {
  PLACEMENT_CENTER_CONF.get_or_init(|| {
    let content = match read_file(config_path) {
      Ok(data) => data,
      Err(e) => {
        panic!("{}", e.to_string())
      }
    };
    let pc_config: PlacementCenterConfig = toml::from_str(&content).unwrap();
    return pc_config;
  })
}

pub fn placement_center_conf() -> &'static PlacementCenterConfig {
  match PLACEMENT_CENTER_CONF.get() {
    Some(config) => {
      return config;
    }
    None => {
      panic!("Placement center configuration is not initialized, check the configuration file.")
    }
  }
}

#[cfg(test)]
mod tests {
  use std::env;

  use crate::config::placement_center::{
    init_placement_center_conf_by_path, placement_center_conf,
  };

  #[test]
  fn config_init_test() {
    let path = format!(
      "{}/../../../config/placement-center.toml",
      env!("CARGO_MANIFEST_DIR")
    );
    println!("{}", path);
    init_placement_center_conf_by_path(&path);
    let config = placement_center_conf();
    assert_eq!(config.node.node_id, 1);
    assert_eq!(config.network.grpc_port, 1228);
  }
}
