pub mod data;

#[derive(Debug, Clone)]
pub struct DataRoute {}

impl DataRoute {
  pub fn build_snapshot(&self) -> Vec<u8> {}

  pub fn recover_snapshot(&self, data: Vec<u8>) -> Result<(), Place> {}
}
