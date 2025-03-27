use std::sync::Arc;

use grpc_clients::pool::ClientPool;
use openraft::RaftNetworkFactory;

use crate::openraft::typeconfig::TypeConfig;

use super::connection::NetworkConnection;

pub struct Network {
  client_pool: Arc<ClientPool>,
}

impl Network {
  pub fn new(client_pool: Arc<ClientPool>) -> Self {
    Self { client_pool }
  }
}

impl RaftNetworkFactory<TypeConfig> for Network {
  type Network = NetworkConnection;

  #[tracing::instrument(level = "debug", skip_all)]
  fn new_client(
    &mut self,
    target: C::NodeId,
    node: &C::Node,
  ) -> impl std::future::Future<Output = Self::Network> + Send {
    todo!()
  }
}
