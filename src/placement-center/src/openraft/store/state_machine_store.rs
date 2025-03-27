use std::sync::Arc;

use openraft::{
  AnyError, ErrorSubject, ErrorVerb, LogId, NodeId, RaftSnapshotBuilder, Snapshot, SnapshotMeta,
  StorageError, StoredMembership, storage::RaftStateMachine,
};
use rocksdb::{BoundColumnFamily, DB};

use crate::{
  openraft::{route::AppResponseData, typeconfig::TypeConfig},
  route::DataRoute,
};

use super::{StorageResult, StoredSnapshot, cf_raft_store};

/// 表示状态机存储的结构体，用于存储状态机的相关数据。
///
/// 该结构体包含以下字段：
/// - `data`: 状态机的数据，包含最后应用的日志ID、最后成员信息和数据路由。
/// - `snapshot_idx`: 快照的索引，用于跟踪快照的版本。
/// - `db`: 一个指向RocksDB数据库的原子引用计数指针，用于持久化存储。
#[derive(Debug, Clone)]
pub struct StateMachineStore {
  // 状态机的数据，包含最后应用的日志ID、最后成员信息和数据路由
  pub data: StateMachineData,
  // 快照的索引，用于跟踪快照的版本
  snapshot_idx: u64,
  // 一个指向RocksDB数据库的原子引用计数指针，用于持久化存储
  db: Arc<DB>,
}

/// 表示状态机数据的结构体，包含状态机所需的核心信息。
///
/// 该结构体包含以下字段：
/// - `last_applied_log_id`: 最后应用的日志ID，用于跟踪状态机处理的日志进度。
/// - `last_membership`: 最后成员信息，存储集群的成员配置。
/// - `route`: 数据路由的原子引用计数指针，用于管理数据的路由规则。
#[derive(Debug, Clone)]
pub struct StateMachineData {
  // 最后应用的日志ID，用于跟踪状态机处理的日志进度
  pub last_applied_log_id: Option<LogId<NodeId>>,
  // 最后成员信息，存储集群的成员配置
  pub last_membership: StoredMembership<TypeConfig>,
  // 数据路由的原子引用计数指针，用于管理数据的路由规则
  pub route: Arc<DataRoute>,
}

impl RaftSnapshotBuilder<TypeConfig> for StateMachineStore {
  async fn build_snapshot(&mut self) -> Result<Snapshot<TypeConfig>, StorageError<TypeConfig>> {
    let last_applied_log = self.data.last_applied_log_id;
    let last_membership = self.data.last_membership.clone();

    let kv_json = self.data.route.build_snapshot();

    let snapshot_id = if let Some(last) = last_applied_log {
      format!("{}-{}-{}", last.leader_id, last.index, self.snapshot_idx)
    } else {
      format!("--{}", self.snapshot_idx)
    };

    let meta = SnapshotMeta {
      last_log_id: last_applied_log,
      last_membership,
      snapshot_id,
    };

    let snapshot = StoredSnapshot {
      meta: meta.clone(),
      data: kv_json.clone(),
    };

    self.set_current_snapshot_(snapshot)?;

    Ok(Snapshot {
      meta,
      snapshot: Box::new(kv_json),
    })
  }
}

impl StateMachineStore {
  pub async fn new(
    db: Arc<DB>,
    route: Arc<DataRoute>,
  ) -> Result<StateMachineStore, StorageError<TypeConfig>> {
    let mut sm = Self {
      data: StateMachineData {
        last_applied_log_id: None,
        last_membership: Default::default(),
        route,
      },
      snapshot_idx: 0,
      db,
    };

    let snapshot = sm.get_current_snapshot_()?;
    if let Some(snap) = snapshot {
      sm.update_state_machine_(snap).await?;
    }

    Ok(sm)
  }

  async fn update_state_machine_(
    &mut self,
    snapshot: StoredSnapshot,
  ) -> Result<(), StorageError<TypeConfig>> {
    self.data.last_applied_log_id = snapshot.meta.last_log_id;
    self.data.last_membership = snapshot.meta.last_membership.clone();

    match self.data.route.recover_snapshot(snapshot.data) {
      Ok(_) => Ok(()),
      Err(e) => Err(StorageError::read(&e)),
    }
  }

  fn get_current_snapshot_(&self) -> StorageResult<Option<StoredSnapshot>> {
    Ok(
      self
        .db
        .get_cf(&self.store, b"snapshot")
        .map_err(|e| StorageError::read(&e))?
        .and_then(|v| serde_json::from_slice(&v).ok()),
    )
  }

  fn set_current_snapshot_(&self, snap: StoredSnapshot) -> StorageResult<()> {
    self.db.put_cf(
      &self.store(),
      b"snapshot",
      serde_json::to_vec(&snap).unwrap().as_slice(),
    )
  }

  fn flush(
    &self,
    subject: ErrorSubject<TypeConfig>,
    verb: ErrorVerb,
  ) -> Result<(), StorageError<TypeConfig>> {
    self
      .db
      .flush_wal(true)
      .map_err(|e| StorageError::new(subject, verb, AnyError::new(&e)))?;
    Ok(())
  }

  fn store(&self) -> Arc<BoundColumnFamily> {
    self.db.cf_handle(&cf_raft_store()).unwrap()
  }
}

impl RaftStateMachine<TypeConfig> for StateMachineStore {
  type SnapshotBuilder = Self;

  async fn applied_state(
    &mut self,
  ) -> Result<(Option<LogId<NodeId>>, StoredMembership<TypeConfig>), StorageError<TypeConfig>> {
    Ok((
      self.data.last_applied_log_id,
      self.data.last_membership.clone(),
    ))
  }

  async fn apply<I>(&mut self, entries: I) -> Result<Vec<AppResponseData>, StorageError<TypeConfig>>
  where
    I: IntoIterator<Item = typ::Entry> + OptionalSend,
    I::IntoIter: OptionalSend,
  {
    todo!()
  }

  fn get_snapshot_builder(
    &mut self,
  ) -> impl std::future::Future<Output = Self::SnapshotBuilder> + Send {
    todo!()
  }

  fn begin_receiving_snapshot(
    &mut self,
  ) -> impl std::future::Future<Output = Result<C::SnapshotData, StorageError<C>>> + Send {
    todo!()
  }

  fn install_snapshot(
    &mut self,
    meta: &SnapshotMeta<C>,
    snapshot: C::SnapshotData,
  ) -> impl std::future::Future<Output = Result<(), StorageError<C>>> + Send {
    todo!()
  }

  fn get_current_snapshot(
    &mut self,
  ) -> impl std::future::Future<Output = Result<Option<Snapshot<C>>, StorageError<C>>> + Send {
    todo!()
  }
}
