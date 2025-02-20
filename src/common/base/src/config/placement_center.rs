use std::sync::OnceLock;

use serde::Deserialize;
use toml::Table;

use crate::tools::read_file;

#[derive(Debug,Deserialize,Clone,Default)]
pub struct PlacementCenterConfig{
    pub cluster_name: String,
    pub addr: String,
    #[serde(default = "default_node_id")]
    pub node_id:u64,
    #[serde(default = "default_grpc_port")]
    pub grpc_port:usize,
    pub nodes:Table,
    pub http_port:usize,
    pub data_path:String,
    pub log:Log,
}

#[derive(Debug,Default,Deserialize,Clone)]
pub struct Log{
    pub log_config:String,
    pub log_path:String,
}

pub fn default_node_id()->u64{
    1
}

pub fn default_grpc_port()->usize{
    9982
}

static PLACEMENT_CENTER_CONF: OnceLock<PlacementCenterConfig>=OnceLock::new();

pub fn init_placement_center_conf_by_path(config_path: &String)->&'static PlacementCenterConfig{
    PLACEMENT_CENTER_CONF.get_or_init(||{
        let content=match read_file(config_path) {
            Ok(data)=>data,
            Err(e)=>{
                panic!("{}",e.to_string())
            }
        };
        let pc_config:PlacementCenterConfig=toml::from_str(&content).unwrap();
        return pc_config;
    })
}

pub fn placement_center_conf()->&'static PlacementCenterConfig{
    match PLACEMENT_CENTER_CONF.get() {
        Some(config)=>{
            return config;
        }
        None=>{
            panic!("Placement center configuration is not initialized, check the configuration file.")
        }
    }
}

#[cfg(test)]
mod tests{
    use std::env;

    use crate::config::placement_center::{init_placement_center_conf_by_path, placement_center_conf};

    #[test]
    fn config_init_test(){
        let path=format!("{}/../../../config/placement-center.toml",env!("CARGO_MANIFEST_DIR"));
        println!("{}",path);
        init_placement_center_conf_by_path(&path);
        let config=placement_center_conf();
        assert_eq!(config.node_id,1);
        assert_eq!(config.grpc_port,1228);
    }
}