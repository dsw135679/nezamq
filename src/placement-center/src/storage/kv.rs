use std::sync::Arc;

use common_base::errors::NezaMQError;

use super::{
  engine::{
    engine_delete_by_cluster, engine_exists_by_cluster, engine_get_by_cluster,
    engine_save_by_cluster,
  },
  rocksdb::RocksDBEngine,
};

pub struct KvStorage {
  rocksdb_engine_handler: Arc<RocksDBEngine>,
}

impl KvStorage {
  pub fn new(rocksdb_engine_handler: Arc<RocksDBEngine>) -> Self {
    KvStorage {
      rocksdb_engine_handler,
    }
  }

  pub fn set(&self, key: String, value: String) -> Result<(), NezaMQError> {
    return engine_save_by_cluster(self.rocksdb_engine_handler.clone(), key, value);
  }

  pub fn delete(&self, key: String) -> Result<(), NezaMQError> {
    return engine_delete_by_cluster(self.rocksdb_engine_handler.clone(), key);
  }

  pub fn get(&self, key: String) -> Result<Option<String>, NezaMQError> {
    match engine_get_by_cluster(self.rocksdb_engine_handler.clone(), key) {
      Ok(Some(data)) => match serde_json::from_slice::<String>(&data.data) {
        Ok(data) => {
          return Ok(Some(data));
        }

        Err(e) => {
          return Err(e.into());
        }
      },
      Ok(None) => {
        return Ok(None);
      }
      Err(e) => Err(e),
    }
  }

  pub fn exists(&self, key: String) -> Result<bool, NezaMQError> {
    return engine_exists_by_cluster(self.rocksdb_engine_handler.clone(), key);
  }
}
