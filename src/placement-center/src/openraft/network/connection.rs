use std::sync::Arc;

use bincode::{deserialize, serialize};
use common_base::error::CommonError;
use grpc_clients::placement::openraft::OpenRaftServiceManager;
use grpc_clients::pool::ClientPool;
use mobc::Connection;
use openraft::RaftNetwork;
use openraft::error::{RPCError, RaftError};
use openraft::network::RPCOption;
use openraft::raft::{AppendEntriesRequest, AppendEntriesResponse};
use protocol::AppendRequest;

use crate::openraft::error::to_error;
use crate::openraft::typeconfig::TypeConfig;

/// 表示网络连接的结构体。
/// 它包含一个地址和一个客户端池。
#[derive(Debug)]
pub struct NetworkConnection {
  // 连接的地址，以字符串形式存储
  addr: String,
  // 客户端池的共享智能指针，用于管理客户端连接
  client_pool: Arc<ClientPool>,
}

impl NetworkConnection {
  /// 创建一个新的 `NetworkConnection` 实例。
  ///
  /// # 参数
  /// - `addr`: 连接的地址，以字符串形式传入。
  /// - `client_pool`: 客户端池的共享智能指针。
  ///
  /// # 返回值
  /// 一个新的 `NetworkConnection` 实例。
  pub fn new(addr: String, client_pool: Arc<ClientPool>) -> Self {
    // 使用传入的地址和客户端池创建一个新的实例
    Self { addr, client_pool }
  }

  // 异步方法，用于获取一个连接。
  // 此方法尚未实现，返回值类型为 `Result<Connection<OpenRaftServiceManager>>`
  async fn c(&mut self) -> Result<Connection<OpenRaftServiceManager>, CommonError> {
    self
      .client_pool
      .placement_center_openraft_service_client(&self.addr)
      .await
  }
}

#[allow(clippy::blocks_in_conditions)]
impl RaftNetwork<TypeConfig> for NetworkConnection {
  async fn append_entries(
    &mut self,
    req: AppendEntriesRequest<TypeConfig>,
    option: RPCOption,
  ) -> Result<AppendEntriesResponse<TypeConfig>, RPCError<TypeConfig, RaftError<TypeConfig>>> {
    let mut c = match self.c().await {
      Ok(conn) => conn,
      Err(e) => return Err(to_error(e)),
    };

    let value = match serialize(&req) {
      Ok(data) => data,
      Err(e) => return Err(to_error(e)),
    };

    let request = AppendRequest { value };
    let reply = match c.append(request).await {
      Ok(reply) => reply.into_inner(),
      Err(e) => return Err(to_error(CommonError::CommonError(r.to_string()))),
    };

    let result = match deserialize(&reply.value) {
      Ok(data) => data,
      Err(e) => return Err(to_error(CommonError::CommonError(e.to_string()))),
    };

    Ok(result)
  }

  #[doc = " Send an InstallSnapshot RPC to the target."]
  fn install_snapshot(
    &mut self,
    _rpc: crate::raft::InstallSnapshotRequest<C>,
    _option: RPCOption,
  ) -> impl std::future::Future<
    Output = Result<
      crate::raft::InstallSnapshotResponse<C>,
      RPCError<C, RaftError<C, crate::error::InstallSnapshotError>>,
    >,
  > + Send {
    todo!()
  }

  #[doc = " Send a RequestVote RPC to the target."]
  fn vote(
    &mut self,
    rpc: VoteRequest<C>,
    option: RPCOption,
  ) -> impl std::future::Future<Output = Result<VoteResponse<C>, RPCError<C, RaftError<C>>>> + Send
  {
    todo!()
  }
}
