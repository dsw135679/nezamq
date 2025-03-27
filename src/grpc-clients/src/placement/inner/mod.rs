use common_base::error::CommonError;
use mobc::Manager;
use protocol::placement_center_server_client::PlacementCenterServerClient;
use tonic::transport::Channel;

pub struct PlacementServiceManager{
    pub addr: String,
}

impl PlacementServiceManager{
    pub fn new(addr:String)->Self{
        Self { addr }
    }
}

#[tonic::async_trait]
impl Manager for PlacementServiceManager {
   
type Connection = PlacementCenterServerClient<Channel>;
type Error=CommonError;


async fn connect(&self) ->  Result<Self::Connection,Self::Error>{
        match PlacementCenterServerClient::connect(format!("http://{}",self.addr.clone())).await {
            Ok(client)=>{
                return Ok(client);
            }
            Err(err)=>{
                return Err(CommonError::CommonError(format!("manager connect error:{},{}",err,self.addr.clone())));
            }
        }
    }


async fn check(&self,conn:Self::Connection) -> Result<Self::Connection,Self::Error>{
        Ok(conn)
    }
}
