pub mod configs;
pub mod handle_file;
pub mod args;
pub mod template_go;
pub mod cmd;

use template_go::template_gin_go::{GoTemplate, self};
use clap::Parser;
use configs::configs::{read_config,Config, HttpType};
use handle_file::handle_file::{mkdir,quick_touch};

fn main() {
    println!("build go projects quickly with rust");
    create_dir_by_config();
}

fn read_cmd_args() -> String {
    let _args = args::args::Args::parse();
    if _args.defines.len() < 1{
        println!("please pass --help else generate default configuration");
        return String::from("")
    }
    print!("{:?}",_args.defines[0].1.clone());
    return _args.defines[0].1.clone()
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
    for dir in cfg.http.project_dir{
        let temp_dir:&str = &(project_name.clone()+&dir);
        mkdir(temp_dir);
    }
    for file in cfg.project_root_file{
        let temp_file:&str = &(project_name.clone()+&file);
        quick_touch(temp_file);
    }
    let path = &(project_name.clone()+&(String::from("/cmd/main.go")));
    GoTemplate::create(String::from("main"),template_gin_go::GIN_TEMPLATE_STR.to_owned(),path.clone()); 
}

