use common_base::error::CommonError;
use mobc::Manager;
use protocol::open_raft_service_client::OpenRaftServiceClient;
use tonic::transport::Channel;

pub mod call;

#[derive(Debug, Clone)]
pub struct OpenRaftServiceManager {
  pub addr: String,
}

impl OpenRaftServiceManager {
  pub fn new(addr: String) -> Self {
    Self { addr }
  }
}

#[tonic::async_trait]
impl Manager for OpenRaftServiceManager {
  type Connection = OpenRaftServiceClient<Channel>;
  type Error = CommonError;

  async fn connect(&self) -> Result<Self::Connection, Self::Error> {
    let addr = format!("http://{}", self.addr.clone());

    match OpenRaftServiceClient::connect(addr.clone()).await {
      Ok(client) => {
        return Ok(client);
      }
      Err(err) => return Err(CommonError::CommonError(format!("{},{}", err, addr))),
    }
  }

  async fn check(&self, conn: Self::Connection) -> Result<Self::Connection, Self::Error> {
    Ok(conn)
  }
}

// impl_retriable_request!(
//   VoteRequest,
//   OpenRaftServiceClient<Channel>,
//   VoteReply,
//   placement_center_openraft_service_client,
//   vote,
//   true
// );

// impl_retriable_request!(
//   AppendRequest,
//   OpenRaftServiceClient<Channel>,
//   AppendReply,
//   placement_center_openraft_service_client,
//   append,
//   true
// );

// impl_retriable_request!(
//   SnapshotRequest,
//   OpenRaftServiceClient<Channel>,
//   SnapshotReply,
//   placement_center_openraft_service_client,
//   snapshot,
//   true
// );

// impl_retriable_request!(
//   AddLearnerRequest,
//   OpenRaftServiceClient<Channel>,
//   AddLearnerReply,
//   placement_center_openraft_service_client,
//   add_learner,
//   true
// );

// impl_retriable_request!(
//   ChangeMembershipRequest,
//   OpenRaftServiceClient<Channel>,
//   ChangeMembershipReply,
//   placement_center_openraft_service_client,
//   change_membership,
//   true
// );
