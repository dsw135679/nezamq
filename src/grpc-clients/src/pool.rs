use common_base::error::CommonError;
use dashmap::{DashMap, mapref::one::Ref};
use mobc::{Connection, Pool};

use crate::placement::{inner::PlacementServiceManager, openraft::OpenRaftServiceManager};

#[derive(Clone, Debug)]
pub struct ClientPool {
  max_open_connection: u64,
  placement_center_leader_addr_caches: DashMap<String, String>,
  placement_center_inner_pools: DashMap<String, Pool<PlacementServiceManager>>,
  placement_center_openraft_service_pools: DashMap<String, Pool<OpenRaftServiceManager>>,
  // placement_center_journal_service_pools:DashMap<String,Pool<JournalServiceManager>>,
  placement_center_kv_service_pools: DashMap<String, Pool<KvServiceManager>>,
}

impl ClientPool {
  pub fn new(max_open_connection: u64) -> Self {
    Self {
      max_open_connection,
      placement_center_leader_addr_caches: DashMap::with_capacity(2),
      placement_center_inner_pools: DashMap::with_capacity(2),
      placement_center_openraft_service_pools: DashMap::with_capacity(2),
      placement_center_kv_service_pools: DashMap::with_capacity(2),
    }
  }

  // -----------------modules: placement center-----------------
  pub async fn placement_center_inner_service_client(
    &self,
    addr: &str,
  ) -> Result<Connection<PlacementServiceManager>, CommonError> {
    if !self.placement_center_inner_pools.contains_key(addr) {
      let manager = PlacementServiceManager::new(addr.to_owned());
      let pool = Pool::builder()
        .max_open(self.max_open_connection)
        .build(manager);
      self
        .placement_center_inner_pools
        .insert(addr.to_owned(), pool);
    }

    if let Some(pool) = self.placement_center_inner_pools.get(addr) {
      match pool.get().await {
        Ok(conn) => return Ok(conn),
        Err(e) => {
          return Err(CommonError::NoAvailableGrpcConnection(
            "PlacementService".to_string(),
            e.to_string(),
          ));
        }
      }
    }

    Err(CommonError::NoAvailableGrpcConnection(
      "PlacementService".to_string(),
      "conncetion pool is not initialized".to_string(),
    ))
  }

  pub async fn placement_center_openraft_service_client(
    &self,
    addr: &str,
  ) -> Result<Connection<OpenRaftServiceManager>, CommonError> {
    if !self
      .placement_center_openraft_service_pools
      .contains_key(addr)
    {
      let manager = OpenRaftServiceManager::new(addr.to_owned());
      let pool = Pool::builder()
        .max_open(self.max_open_connection)
        .build(manager);
      self
        .placement_center_openraft_service_pools
        .insert(addr.to_owned(), pool);
    }

    if let Some(pool) = self.placement_center_openraft_service_pools.get(addr) {
      match pool.get().await {
        Ok(conn) => return Ok(conn),
        Err(e) => {
          return Err(CommonError::NoAvailableGrpcConnection(
            "OpenRaftService".to_string(),
            e.to_string(),
          ));
        }
      }
    }

    Err(CommonError::NoAvailableGrpcConnection(
      "OpenRaftService".to_string(),
      "conncetion pool is not initialized".to_string(),
    ))
  }

  pub fn get_leader_addr(&self, addr: &str) -> Option<Ref<'_, String, String>> {
    self.placement_center_leader_addr_caches.get(addr)
  }

  pub fn set_leader_addr(&self, addr: String, leader_addr: String) {
    self
      .placement_center_leader_addr_caches
      .insert(addr.to_owned(), leader_addr.to_owned());
  }
}
