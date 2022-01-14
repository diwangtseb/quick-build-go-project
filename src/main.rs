pub mod configs;
pub mod handle_file;

use std::env;
use configs::{configs::{read_config,Config}};
use handle_file::{handle_file::{mkdir,quick_touch}};

fn main() {
    println!("Build go projects quickly with Rust");
    create_dir_by_config()
}

fn read_cmd_args() -> String{
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);
    if args.len() < 3 {
        return String::from("");
    };
    if args[2] == "" {
        panic!("{:?}",args)
    };
    args[2].clone()
}

fn create_dir_by_config(){
    let cfg:Config = read_config();
    let project_name: String;
    if read_cmd_args() != ""{
        project_name = read_cmd_args();
    }else{
        project_name = cfg.project_name;
    }
    mkdir(&project_name);
    for dir in cfg.project_dir{
        let temp_dir:&str = &(project_name.clone()+&dir);
        mkdir(temp_dir);
    }
    for file in cfg.project_root_file{
        let temp_file:&str = &(project_name.clone()+&file);
        quick_touch(temp_file);
    }
}
