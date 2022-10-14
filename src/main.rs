pub mod configs;
pub mod handle_file;
pub mod args;
pub mod template_go;

use template_go::{template_go::GoTemplate};
use clap::Parser;
use configs::{configs::{read_config,Config}};
use handle_file::{handle_file::{mkdir,quick_touch}};

fn main() {
    println!("Build go projects quickly with Rust");
    create_dir_by_config();
}

fn read_cmd_args() -> String {
    let _args = args::args::Args::parse();
    if _args.defines.len() < 1{
        println!("please pass --help else generate default configuration");
        return String::from("")
    }
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
    for dir in cfg.project_dir{
        let temp_dir:&str = &(project_name.clone()+&dir);
        mkdir(temp_dir);
    }
    for file in cfg.project_root_file{
        let temp_file:&str = &(project_name.clone()+&file);
        quick_touch(temp_file);
    }
    let path = &(project_name.clone()+&(String::from("/cmd/main.go")));
    GoTemplate::create(String::from("main"),String::from(r#"
package main

import "github.com/gin-gonic/gin"

func main() {
	r := gin.Default()
	r.GET("/ping", func(c *gin.Context) {
		c.JSON(200, gin.H{
			"message": "pong",
		})
	})
	r.Run() // listen and serve on 0.0.0.0:8080 (for windows "localhost:8080")
}
            "#),path.clone()); 
}

