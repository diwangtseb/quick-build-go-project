use std::process::Command;

pub const KRATOS_LAYOUT: &str = "https://github.com/diwangtseb/foo.git";

fn create_kratos_project(project_name: &str) {
    //currently, only unix is supported
    let _r = Command::new("kratos")
        .args(["new", project_name, "-r", KRATOS_LAYOUT])
        .output()
        .expect("create kratos proj failed");
}


#[cfg(test)]

mod tests {

    use super::create_kratos_project;
    #[test]
    fn test_create_kratos_project() {
        create_kratos_project("zs");
    }
}