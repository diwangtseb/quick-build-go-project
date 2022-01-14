extern crate serde_yaml;
extern crate serde;

use serde::{Serialize,Deserialize};


#[derive(Debug,Serialize,Deserialize)]
pub struct Config {
   pub project_name: String,
   pub version: String,
   pub project_dir: Vec<String>,
   pub project_root_file: Vec<String>
}

pub fn read_config() -> Config{
    let yaml_str = include_str!("./config.yaml");
    let config: Config = serde_yaml::from_str(yaml_str).expect("yaml read field!");
    return config;
}
