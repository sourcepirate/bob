use std::io;
use std::env::set_current_dir;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use builder::{Packable, SourceTypes, TargetTypes};
use glob::{glob, Paths};
use fpm::Fpm;

pub struct Ruby;

impl Ruby {
    pub fn new() -> Self {Ruby{}}
}

fn list_gem_files(pac: String) -> Paths{
    glob(&pac[..]).unwrap()
}

impl Packable for Ruby {
    fn build(&self, dir_path: String, target_type: TargetTypes, packagename: String, outpath: String) -> Option<i32> {
        let build_path :&Path = Path::new(&dir_path);
        let build_type = SourceTypes::GEM;
        let switched = set_current_dir(&build_path).is_ok();
        if !switched{
            return Some(2)
        }

        info!("Searching for gemspec file");

        let mut spec_name = packagename.clone();
        spec_name.push_str(".gemspec");

        info!("Trying to find the specfile");

        let mut gem_command = Command::new("gem")
            .arg("build").arg(&spec_name[..]).status().unwrap();

        if !gem_command.success(){
            return Some(2);
        }

        let mut gem_name = packagename.clone();
        gem_name.push_str("-*.gem");

        let mut dot_gem_files: Paths = list_gem_files(gem_name);
        let gem = dot_gem_files.nth(0)
            .and_then(|p| Some(p.ok()))
            .and_then(|p| Some(p.unwrap()))
            .and_then(|p| Some(p.into_os_string()))
            .and_then(|p| Some(p.into_string()))
            .unwrap();

        let gem_new_name :String = if gem.is_ok() {
            gem.unwrap()
        } else {
            return Some(2)
        };

        let mut r_fpm = Fpm::new(build_type.as_string(), target_type.as_string());
        r_fpm.file(gem_new_name);
        r_fpm.outdir(outpath);
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
