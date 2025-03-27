use common_base::error::CommonError;
use mobc::Manager;
use protocol::kv_service_client::KvServiceClient;
use tonic::transport::Channel;

pub mod call;

#[derive(Debug, Clone)]
pub struct KvServiceManager {
  pub addr: String,
}

impl KvServiceManager {
  pub fn new(addr: String) -> Self {
    Self { addr }
  }
}

#[tonic::async_trait]
impl Manager for KvServiceManager {
  type Connection = KvServiceClient<Channel>;
  type Error = CommonError;

  async fn connect(&self) -> Result<Self::Connection, Self::Error> {
    match KvServiceClient::connect(format!("http://{}", self.addr.clone())).await {
      Ok(client) => {
        return Ok(client);
      }
      Err(err) => {
        return Err(CommonError::CommonError(format!(
          "{},{}",
          err,
          self.addr.clone()
        )));
      }
    }
  }

  async fn check(&self, conn: Self::Connection) -> Result<Self::Connection, Self::Error> {
    Ok(conn)
  }
}
