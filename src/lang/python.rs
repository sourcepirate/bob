
use std::io;
use std::env::set_current_dir;
use std::path::Path;
use std::process::{Command, Stdio};
use builder::{Packable, SourceTypes, TargetTypes};
use fpm::Fpm;

pub struct Python;

impl Packable for Python {
    fn build(&self, dir_path: String, target_type: TargetTypes, packagename: String) -> Option<i32> {
        let build_path :&Path = Path::new(&dir_path);
        let build_type = SourceTypes::PYTHON;
        let switched = set_current_dir(&build_path).is_ok();
        if !switched{
            return Some(2)
        }

        let mut r_fpm = Fpm::new(build_type.as_string(), target_type.as_string());
        r_fpm.file("setup.py".to_string());
        r_fpm.name(packagename);

        match r_fpm.execute(){
            Ok(status) => {
                if status.success(){
                    Some(0)
                } else {
                    status.code()
                }
            },
            Err(_) => Some(2)
        }
    }
}
