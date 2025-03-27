use crate::{
  config::placement_center::placement_center_conf,
  tools::{create_fold, file_exists, read_file},
};

pub fn init_placement_center_log() {
  // 1. 获取配置信息
  let conf = placement_center_conf();
  // 2. 检查日志配置 .yaml 文件是否存在
  if !file_exists(&conf.log.log_config) {
    panic!(
      "Logging configuration file {} does not exists",
      conf.log.log_config
    );
  }
  // 3. 尝试初始化日志存放目录
  match create_fold(&conf.log.log_path) {
    Ok(()) => {}
    Err(_) => {
      panic!("Failed to initialize log directory {}", conf.log.log_path);
    }
  }
  // 4. 读取日志配置 .yaml 文件的内容
  let content = match read_file(&conf.log.log_config) {
    Ok(data) => data,
    Err(e) => {
      panic!("{}", e.to_string());
    }
  };
  // 5. 替换日志文件的存放路径
  let config_content = content.replace("{}", &conf.log.log_path);
  println!("{}", "log config:");
  println!("{}", config_content);
  // 6. 解析 yaml 格式的配置文件
  let config = match serde_yaml::from_str(&config_content) {
    Ok(data) => data,
    Err(e) => {
      panic!(
        "Failed to parse the contents of the config file {} with error message: {}",
        conf.log.log_config,
        e.to_string()
      );
    }
  };
  // 7. 初始化日志配置
  match log4rs::init_raw_config(config) {
    Ok(_) => {}
    Err(e) => {
      panic!("{}", e.to_string());
    }
  }
}

#[cfg(test)]
mod tests {}
