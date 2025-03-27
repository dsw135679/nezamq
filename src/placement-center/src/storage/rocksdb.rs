use std::{collections::HashMap, path::Path};

use common_base::{config::placement_center::PlacementCenterConfig, errors::NezaMQError};
use rocksdb::{ColumnFamily, DB, DBCompactionStyle, Options, SliceTransform};
use serde::{Serialize, de::DeserializeOwned};

pub const DB_COLUMN_FAMILY_CLUSTER: &str = "cluster";

pub struct RocksDBEngine {
  pub db: DB,
}

impl RocksDBEngine {
  // 创建RocksDB 实例
  pub fn new(config: &PlacementCenterConfig) -> Self {
    // 1. 设置 RocksDB 配置参数
    let opts = Self::set_db_opts();
    let db_path = format!("{}/{}", config.data_path, "_storage_rocksdb");
    // 2. 初始化 RocksDB 实例：判断RocksDB是否初始化成功，否则进行初始化。
    if !Path::new(&db_path).exists() {
      DB::open(&opts, db_path.clone()).unwrap();
    }
    // 3. 初始化 RocksDB 中的列蔟
    let cf_list = DB::list_cf(&opts, &db_path).unwrap();
    let instance = DB::open_cf(&opts, db_path.clone(), &cf_list).unwrap();

    Self { db: instance }
  }

  // RocksDB 配置设置
  fn set_db_opts() -> Options {
    let mut opts = Options::default();
    opts.create_if_missing(true);
    opts.create_missing_column_families(true);
    opts.set_max_open_files(1000);
    opts.set_use_fsync(false);
    opts.set_bytes_per_sync(8388608);
    opts.optimize_for_point_lookup(1024);
    opts.set_table_cache_num_shard_bits(6);
    opts.set_max_write_buffer_number(32);
    opts.set_write_buffer_size(536870912);
    opts.set_target_file_size_base(1073741824);
    opts.set_min_write_buffer_number_to_merge(4);
    opts.set_level_zero_stop_writes_trigger(2000);
    opts.set_level_zero_slowdown_writes_trigger(0);
    opts.set_compaction_style(DBCompactionStyle::Universal);
    opts.set_disable_auto_compactions(true);

    let transform = SliceTransform::create_fixed_prefix(10);
    opts.set_prefix_extractor(transform);
    opts.set_memtable_prefix_bloom_ratio(0.2);

    return opts;
  }
}

impl RocksDBEngine {
  // 写数据（Write）
  pub fn write<T: Serialize + std::fmt::Debug>(
    &self,
    cf: &ColumnFamily,
    key: &str,
    value: &T,
  ) -> Result<(), String> {
    match serde_json::to_string(&value) {
      Ok(serialized) => self
        .db
        .put_cf(cf, key, serialized.into_bytes())
        .map_err(|err| format!("Failed to put to ColumnFamily:{:?}", err)),
      Err(err) => Err(format!(
        "Failed to Serialize to String. T:{:?},err:{:?}",
        value, err
      )),
    }
  }

  // 根据 key 读取数据
  pub fn read<T: DeserializeOwned>(
    &self,
    cf: &ColumnFamily,
    key: &str,
  ) -> Result<Option<T>, String> {
    match self.db.get_cf(cf, key) {
      Ok(opt) => match opt {
        Some(found) => match String::from_utf8(found) {
          Ok(s) => match serde_json::from_str::<T>(&s) {
            Ok(t) => Ok(Some(t)),
            Err(err) => Err(format!("Failed to deserialize: {:?}", err)),
          },
          Err(err) => Err(format!("Failed to deserialize: {:?}", err)),
        },
        None => Ok(None),
      },
      Err(err) => Err(format!("Failed to get from ColumnFamily: {:?}", err)),
    }
  }

  // 根据 key 删除数据
  pub fn delete(&self, cf: &ColumnFamily, key: &str) -> Result<(), NezaMQError> {
    return Ok(self.db.delete_cf(cf, key)?);
  }

  // 根据 key 是否存在
  pub fn exist(&self, cf: &ColumnFamily, key: &str) -> bool {
    self.db.key_may_exist_cf(cf, key)
  }

  // 根据 key 前缀搜索
  pub fn read_prefix(&self, cf: &ColumnFamily, search_key: &str) -> Vec<HashMap<String, Vec<u8>>> {
    // 获取 ColumnFamily 的迭代器
    let mut iter = self.db.raw_iterator_cf(cf);

    // 搜索到第一个匹配这个前缀的 Key
    iter.seek(search_key);

    let mut result = Vec::new();

    // 获取下一个 key 的值
    while iter.valid() {
      let key = iter.key();
      let value = iter.value();

      let mut raw = HashMap::new();

      // 如果 key 和 value 都为空，则退出循环
      if key == None || value == None {
        break;
      }

      let result_key = match String::from_utf8(key.unwrap().to_vec()) {
        Ok(s) => s,
        Err(_) => continue,
      };

      // 如果 key 不匹配前缀，说明已经获取到所有这个前缀的 key,则退出循环。
      if !result_key.starts_with(search_key) {
        break;
      }

      raw.insert(result_key, value.unwrap().to_vec());
      result.push(raw);
      iter.next();
    }

    return result;
  }

  pub fn cf_cluster(&self) -> &ColumnFamily {
    return self.db.cf_handle(&DB_COLUMN_FAMILY_CLUSTER).unwrap();
  }
}
