use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

#[derive(Debug)]
pub struct GoTemplate {
    pub package_name: String,
    pub default_content: String,
}

impl GoTemplate {
    pub fn create(pkg_name:String,default_content:String,path:String) {
        let gt = GoTemplate {
            package_name: pkg_name,
            default_content:default_content
        }; 
        let _path = Path::new(&path);
        let display = _path.display();
        let mut file = File::create(&_path).unwrap().write_all(gt.default_content.as_bytes());
    }
}
