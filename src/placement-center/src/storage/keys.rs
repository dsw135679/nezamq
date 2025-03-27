/** =========Raft============= */
pub fn key_name_by_first_index()->String{
  return "/raft/first_index".to_string();
}

pub fn key_name_by_last_index()->String{
  return "/raft/last_index".to_string();
}

pub fn key_name_by_hard_state()->String{
  return "/raft/hard_state".to_string();
}

pub fn key_name_by_conf_state()->String{
  return "/raft/conf_state".to_string();
}

pub fn key_name_by_entry(idx: u64)->String{
  return format!("/raft/entry/{}",idx);
}

pub fn key_name_uncommit()->String{
  return "/raft/uncommit_index".to_string();
}

pub fn key_name_snapshot()->String{
  return "/raft/snapshot".to_string();
}

/** ======Cluster============ */
// 用户User的key
pub fn storage_key_mqtt_user(cluster_name: &String, user_name: &String) -> String {
  return format!("/mqtt/user/{}/{}", cluster_name, user_name);
}

// 用户使用前缀搜索
pub fn storage_key_mqtt_user_cluster_prefix(cluster_name: &String) -> String {
  return format!("/mqtt/user/{}", cluster_name);
}

pub fn storage_key_mqtt_topic(cluster_name: &String, user_name: &String) -> String {
  return format!("/mqtt/topic/{}/{}", cluster_name, user_name);
}

pub fn storage_key_mqtt_topic_prefix(cluster_name: &String) -> String {
  return format!("/mqtt/topic/{}", cluster_name);
}
