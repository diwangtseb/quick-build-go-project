extern crate serde_yaml;
extern crate serde;
extern crate serde_json;

use serde::{Serialize,Deserialize};


#[derive(Serialize, Deserialize)]
pub struct Config{
    project_name: String,
    version: String,
    http: Http,
    grpc: Option<serde_json::Value>,
    project_root_file: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Http {
    project_dir: Vec<String>,
}


pub fn read_config() -> Config{
    let yaml_str = include_str!("./config.yaml");
    let config: Config = serde_yaml::from_str(yaml_str).expect("yaml read field!");
    return config;
}
