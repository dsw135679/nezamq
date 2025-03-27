use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppResponseData {
  pub value: Option<Vec<u8>>,
}
