use std::{path::Path, sync::Arc};

use bincode::config::BigEndian;
use log_store::LogStore;
use openraft::{SnapshotMeta, StorageError};
use serde::{Deserialize, Serialize};
use tokio::io::AsyncReadExt;

use super::typeconfig::TypeConfig;

pub mod log_store;
pub mod state_machine_store;

/// 储存返回结果
type StorageResult<T> = Result<T, StorageError<TypeConfig>>;

/// 存储快照
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredSnapshot {
  pub meta: SnapshotMeta<TypeConfig>,

  /// 快照数据
  pub data: Vec<u8>,
}

/// 大端字节序（Big Endian），也称为网络字节序，是一种在多字节数据存储和传输时，将最高有效字节（MSB, Most Significant Byte）存放在最低内存地址，
/// 而将最低有效字节（LSB, Least Significant Byte）存放在最高内存地址的字节序排列方式。
/// 例如，对于一个16位整数0x1234，在大端字节序中，内存中的存储顺序是0x12（高字节）在前，0x34（低字节）在后。
/// 大端字节序的主要优点是：人类在书写多位数时，通常也是从高位到低位书写，因此大端字节序的表示方式与人类的习惯相符。
/// 同时，在比较两个多字节整数时，大端字节序可以从左到右按字节顺序比较，符合自然的比较逻辑，有利于确保键的正确排序。
/// 在网络通信中，大端字节序被广泛采用，因为它可以保证不同系统之间的数据传输和解析的一致性。
/// 将 id 转为大端字节序
/// 将一个64位无符号整数转换为大端字节序的字节数组。
///
/// 该函数接受一个64位无符号整数 `id` 作为输入，并将其转换为大端字节序的字节数组。
/// 使用 `Vec::with_capacity` 预先分配8字节的空间，以提高性能。
///
/// # 参数
/// - `id`: 要转换的64位无符号整数。
///
/// # 返回值
/// 返回一个包含大端字节序的字节数组 `Vec<u8>`。
fn id_to_bin(id: u64) -> Vec<u8> {
  // 预先分配8字节的空间，避免后续扩容带来的性能开销
  let mut buf = Vec::with_capacity(8);
  // 将 `id` 以大端字节序写入 `buf` 中
  buf.write_u64::<BigEndian>(id).unwrap();
  // 返回包含大端字节序的字节数组
  buf
}

/// 将大端字节序的字节数组转换为64位无符号整数。
///
/// 该函数接受一个字节切片 `buf` 作为输入，并将其前8个字节解析为大端字节序的64位无符号整数。
///
/// # 参数
/// - `buf`: 包含大端字节序的字节切片。
///
/// # 返回值
/// 返回一个解析后的64位无符号整数 `u64`。
fn bin_to_id(buf: &[u8]) -> u64 {
  // 从 `buf` 的前8个字节中读取大端字节序的64位无符号整数
  (&buf[0..8]).read_u64::<BigEndian>().unwrap()
}

pub(crate) async fn new_storage<P: AsRef<Path>>(
  db_path: P,
  route: Arc<DataRoute>,
) -> (LogStore, StateMachineStore) {
}

/// 返回Raft存储的列族名称
fn cf_raft_store() -> String {
  "store".to_string()
}

/// 返回Raft日志的列族名称
fn cf_raft_logs() -> String {
  "logs".to_string()
}
