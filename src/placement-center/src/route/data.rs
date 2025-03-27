use core::fmt;

use serde::{Deserialize, Serialize};

// 为结构体添加派生属性，支持调试输出、序列化和反序列化
#[derive(Debug, Deserialize, Serialize)]
/// 表示存储数据的结构体，包含数据类型和数据值
pub struct StorageData {
  /// 数据类型，使用 `StorageDataType` 枚举
  pub data_type: StorageDataType,
  /// 数据值，以字节向量的形式存储
  pub value: Vec<u8>,
}

impl StorageData {
  /// 创建一个新的 `StorageData` 实例
  ///
  /// # 参数
  /// - `data_type`: 存储数据的类型，使用 `StorageDataType` 枚举
  /// - `value`: 存储数据的值，以字节向量的形式存储
  ///
  /// # 返回值
  /// 返回一个新的 `StorageData` 实例
  pub fn new(data_type: StorageDataType, value: Vec<u8>) -> StorageData {
    // 初始化并返回一个新的 `StorageData` 实例
    StorageData { data_type, value }
  }
}

impl fmt::Display for StorageData {
  /// 实现 `fmt::Display` trait，用于格式化输出 `StorageData` 实例
  ///
  /// # 参数
  /// - `self`: `StorageData` 实例的引用
  /// - `f`: 格式化输出的目标 `Formatter`
  ///
  /// # 返回值
  /// 返回一个 `fmt::Result` 类型的结果，表示格式化输出是否成功
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    // 格式化输出数据类型和数据值
    write!(f, "({:?},{:?})", self.data_type, self.value)
  }
}

// 为枚举添加派生属性，支持调试输出、序列化和反序列化
#[derive(Debug, Deserialize, Serialize)]
/// 表示存储数据的类型的枚举，包含不同的操作类型
pub enum StorageDataType {
  // KV 操作类型
  /// 表示设置键值对的操作
  KvSet,
  /// 表示删除键值对的操作
  KvDelete,

  // 集群操作类型
  /// 表示向集群中添加节点的操作
  ClusterAddNode,
  /// 表示从集群中删除节点的操作
  ClusterDeleteNode,
  /// 表示添加一个新集群的操作
  ClusterAddCluster,
  /// 表示删除一个集群的操作
  ClusterDeleteCluster,
}
