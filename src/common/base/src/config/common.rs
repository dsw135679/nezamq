use std::{collections::HashMap, env};

use serde::{Deserialize, Serialize};

/// 定义存储配置结构体，包含存储类型和不同存储方式的地址等信息
///
/// # 字段
/// - `storage_type`: 存储类型
/// - `journal_addr`: 日志存储地址，默认为空字符串
/// - `mysql_addr`: MySQL 存储地址，默认为空字符串
/// - `rocksdb_data_path`: RocksDB 数据存储路径，默认为空字符串
/// - `rocksdb_max_open_files`: RocksDB 最大打开文件数，可选值
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct Storage {
  // 存储类型
  pub storage_type: String,
  // 日志存储地址，使用 serde 的默认值特性
  #[serde(default)]
  pub journal_addr: String,
  // MySQL 存储地址，使用 serde 的默认值特性
  #[serde(default)]
  pub mysql_addr: String,
  // RocksDB 数据存储路径，使用 serde 的默认值特性
  #[serde(default)]
  pub rocksdb_data_path: String,
  // RocksDB 最大打开文件数，可选值
  pub rocksdb_max_open_files: Option<i32>,
}

/// 定义 Prometheus 监控配置结构体，包含是否启用、模式、端口等信息
///
/// # 字段
/// - `enable`: 是否启用 Prometheus 监控，默认为 false
/// - `model`: Prometheus 监控模式，默认为空字符串
/// - `port`: Prometheus 监控端口，默认为 9090
/// - `push_gateway_server`: Prometheus Push Gateway 服务器地址，默认为空字符串
/// - `interval`: 监控数据推送间隔，默认为 0
/// - `header`: 监控数据请求头，默认为空字符串
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Prometheus {
  // 是否启用 Prometheus 监控，使用 serde 的默认值特性
  #[serde(default)]
  pub enable: bool,
  // Prometheus 监控模式，使用 serde 的默认值特性
  #[serde(default)]
  pub model: String,
  // Prometheus 监控端口，使用默认函数提供默认值
  #[serde(default = "default_prometheus_port")]
  pub port: u32,
  // Prometheus Push Gateway 服务器地址，使用 serde 的默认值特性
  #[serde(default)]
  pub push_gateway_server: String,
  // 监控数据推送间隔，使用 serde 的默认值特性
  #[serde(default)]
  pub interval: u32,
  // 监控数据请求头，使用 serde 的默认值特性
  #[serde(default)]
  pub header: String,
}

/// 定义认证配置结构体，包含存储类型和不同存储方式的地址等信息
///
/// # 字段
/// - `storage_type`: 存储类型
/// - `journal_addr`: 日志存储地址，默认为空字符串
/// - `mysql_addr`: MySQL 存储地址，默认为空字符串
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
pub struct Auth {
  // 存储类型
  pub storage_type: String,
  // 日志存储地址，使用 serde 的默认值特性
  #[serde(default)]
  pub journal_addr: String,
  // MySQL 存储地址，使用 serde 的默认值特性
  #[serde(default)]
  pub mysql_addr: String,
}

/// 定义日志配置结构体，包含日志配置文件路径和日志存储路径
///
/// # 字段
/// - `log_config`: 日志配置文件路径
/// - `log_path`: 日志存储路径
#[derive(Debug, Deserialize, Serialize, Clone, Default, PartialEq, Eq)]
pub struct Log {
  // 日志配置文件路径
  pub log_config: String,
  // 日志存储路径
  pub log_path: String,
}

/// 定义遥测配置结构体，包含是否启用、导出器类型和导出器端点等信息
///
/// # 字段
/// - `enable`: 是否启用遥测功能，默认为 false
/// - `exporter_type`: 遥测数据导出器类型
/// - `exporter_endpoint`: 遥测数据导出器端点
#[derive(Debug, Default, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Telemetry {
  // 是否启用遥测功能
  pub enable: bool,
  // 遥测数据导出器类型
  pub exporter_type: String,
  // 遥测数据导出器端点
  pub exporter_endpoint: String,
}

/// 生成默认的 Prometheus 配置
///
/// # 返回值
/// 返回一个默认配置的 `Prometheus` 结构体实例
pub fn default_prometheus() -> Prometheus {
  // 返回默认配置的 Prometheus 结构体实例
  Prometheus {
    // 禁用 Prometheus 监控
    enable: false,
    // 监控模式为 pull
    model: "pull".to_string(),
    // 使用默认端口
    port: default_prometheus_port(),
    // 空的 Push Gateway 服务器地址
    push_gateway_server: "".to_string(),
    // 推送间隔为 10
    interval: 10,
    // 空的请求头
    header: "".to_string(),
  }
}

/// 返回默认的 Prometheus 端口
///
/// # 返回值
/// 返回默认的 Prometheus 端口号 9090
pub fn default_prometheus_port() -> u32 {
  // 返回默认端口 9090
  9090
}

/// 根据环境变量覆盖 TOML 配置文件中的默认值
///
/// # 参数
/// - `toml_content`: TOML 配置文件的内容
/// - `env_prefix`: 环境变量的前缀
///
/// # 返回值
/// 返回经过环境变量覆盖后的 TOML 配置文件内容
pub fn override_default_by_env(toml_content: String, env_prefix: &str) -> String {
  // 逐行解析配置文件，生成环境变量键名与行号映射
  let mut sub_key = String::new(); // 当前子键
  let mut env_map = HashMap::new();
  for (line_num, line) in toml_content.lines().enumerate() {
    let trimmed = line.trim().replace(" ", "");
    if trimmed.is_empty() || trimmed.starts_with('#') {
      continue; //跳过空行、表头或注释行
    }
    if trimmed.starts_with('[') {
      sub_key = trimmed[1..trimmed.len() - 1].to_string();
      continue;
    }
    if trimmed.is_empty() {
      let (key, _) = trimmed.split_once('=').unwrap();
      let env_key = format!("{}_{}", env_prefix, key.to_uppercase().replace('.', "_"));
      env_map.insert(env_key, line_num);
    } else {
      let (key, _) = trimmed.split_once('=').unwrap();
      let env_key = format!(
        "{}_{}_{}",
        env_prefix,
        sub_key.to_uppercase(),
        key.to_uppercase().replace('.', "_")
      );
      env_map.insert(env_key, line_num);
    }
  }

  // 遍历环境变量映射，查找并替换
  let mut lines: Vec<String> = toml_content.lines().map(|line| line.to_string()).collect();
  for (env_key, line_num) in &env_map {
    if let Ok(env_value) = env::var(env_key) {
      let key = lines[*line_num].split("=").collect::<Vec<&str>>()[0];
      lines[*line_num] = key.to_string() + "=" + &env_value;
    }
  }

  // 重新拼接修改后的 TOML 内容
  lines.join("\n")
}

#[cfg(test)]
mod tests {
  /// 测试 `override_default_by_env` 函数是否能正确根据环境变量覆盖 TOML 配置
  #[test]
  fn override_default_by_env() {
    // 定义一个简单的 TOML 配置文件内容
    let toml_content = r#"
        [server]
        port=8080
        "#;
    // 定义环境变量前缀
    let env_prefix = "APP";
    // 不安全地设置环境变量
    unsafe {
      std::env::set_var("APP_SERVER_PORT", "8081");
    }
    // 调用 `override_default_by_env` 函数进行测试
    let new_toml_content = super::override_default_by_env(toml_content.to_string(), env_prefix);
    // 断言修改后的 TOML 内容是否符合预期
    assert_eq!(
      new_toml_content,
      r#"
        [server]
        port=8081
        "#
    );
  }
}
