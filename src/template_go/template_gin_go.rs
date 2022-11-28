use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

#[derive(Debug)]
pub struct GoTemplate {
    pub package_name: String,
    pub default_content: String,
}

impl GoTemplate {
    pub fn create(pkg_name:String,mut default_content:String,path:String) {
        //now default_content is gin_template_str
        default_content = GIN_TEMPLATE_STR.to_owned();
        let gt = GoTemplate {
            package_name: pkg_name,
            default_content:default_content
        }; 
        let _path = Path::new(&path);
        let _= _path.display();
        let _ = File::create(&_path).unwrap().write_all(gt.default_content.as_bytes()).unwrap();
    }
}
pub const GIN_TEMPLATE_STR:&str = r#"
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
            "#;

