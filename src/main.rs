pub mod configs;
pub mod handle_file;

use configs::{configs::{read_config,Config}};
use handle_file::{handle_file::{mkdir,quick_touch}};

fn main() {
    println!("Build go projects quickly with Rust");
    create_dir()
}

fn create_dir(){
    // create project root dir
    let cfg:Config  = read_config();
    let name:String = cfg.project_name;
    let dir_cmd = name.clone() + "/cmd";
    let dir_internal = name.clone() + "/internal";
    let dir_pkg = name.clone() + "/pkg";
    let dir_vendor = name.clone() + "/vendor";
    let dir_build = name.clone() + "/build";
    let dir_configs = name.clone() + "/configs";
    let dir_deploy = name.clone() + "/deploy";
    let dir_init = name.clone() + "/init";
    let dir_script = name.clone() + "/scripts";
    let dir_tests = name.clone() + "/tests";
    let dir_docs = name.clone() + "/docs";
    let dir_examples = name.clone() + "/examples";
    let dir_githooks = name.clone() + "/githooks";
    let dir_tools = name.clone() + "/tools";
    let dir_third_party = name.clone() + "/third_party";
    mkdir(&name);
    mkdir(&dir_cmd);
    mkdir(&dir_internal);
    mkdir(&dir_pkg);
    mkdir(&dir_vendor);
    mkdir(&dir_build);
    mkdir(&dir_configs);
    mkdir(&dir_deploy);
    mkdir(&dir_init);
    mkdir(&dir_script);
    mkdir(&dir_tests);
    mkdir(&dir_docs);
    mkdir(&dir_examples);
    mkdir(&dir_githooks);
    mkdir(&dir_tools);
    mkdir(&dir_third_party);
}
