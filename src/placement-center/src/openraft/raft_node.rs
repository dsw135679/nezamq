use std::{collections::BTreeMap, fmt::Display};

use common_base::config::placement_center::placement_center_conf;

use super::typeconfig::TypeConfig;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Default)]
pub struct Node {
  pub node_id: u64,
  pub rpc_addr: String,
}

impl Display for Node {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "Node {{rpc_addr: {},node_id: {} }}",
      self.rpc_addr, self.node_id
    )
  }
}

pub mod typ {
  use crate::openraft::typeconfig::TypeConfig;

  pub type Entry = openraft::Entry<TypeConfig>;
}

pub async fn start_openraft_node(raft_node: Raft<TypeConfig>) {
  let conf = placement_center_conf();
  let mut nodes = BTreeMap::new();
  for (node_id, addr) in conf.node.nodes.clone() {}
}
