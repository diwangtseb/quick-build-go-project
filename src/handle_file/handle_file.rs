use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::path::Path;

fn main() {
    println!("Build go projects quickly with Rust about FileCreate");
}

pub fn mkdir(dir_name: &str){
    fs::create_dir_all(dir_name).unwrap_or_else(|why|{
        println!("!{:?}",why.kind());
    });
}

fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new().create(true).write(true).open(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn quick_touch(file_name:&str){
    touch(&Path::new(file_name)).unwrap_or_else(|why| {
        println!("!{:?}", why.kind());
    });
}
