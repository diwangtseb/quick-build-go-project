extern crate serde_yaml;
extern crate serde;
extern crate serde_json;

use serde::{Serialize,Deserialize};


#[derive(Serialize, Deserialize)]
pub struct Config{
    pub project_name: String,
    pub version: String,
    pub http: Http,
    pub grpc: Option<serde_json::Value>,
    pub project_root_file: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Http {
    pub project_dir: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Grpc {
    pub grpc_type: Vec<String>,
}

pub enum HttpType {
    HTTP,
    GRPC,
}

pub fn read_config() -> Config{
    let yaml_str = include_str!("./config.yaml");
    let config: Config = serde_yaml::from_str(yaml_str).expect("yaml read field!");
    return config;
}
