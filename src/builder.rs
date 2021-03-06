
use std::process::Command;


pub enum SourceTypes{
    DIR,
    GEM,
    PYTHON,
    TAR
}

#[derive(Debug, Clone, Copy)]
pub enum TargetTypes {
    DEB,
    RPM
}

pub trait Packable {
    fn build(&self, dir_path:String, target_type: TargetTypes, packagename: String, outpath: String) -> Option<i32>;
}

pub struct Folder{
    pub path: String,
    pub lang: Box<Packable>
}

impl SourceTypes{
    pub fn as_string(&self) -> String {
        match *self {
            SourceTypes::DIR => String::from("dir"),
            SourceTypes::GEM => String::from("gem"),
            SourceTypes::PYTHON => String::from("python"),
            SourceTypes::TAR => String::from("tar")
        }
    }
}

impl TargetTypes {
    pub fn as_string(&self) -> String {
        match *self {
            TargetTypes::DEB => String::from("dir"),
            TargetTypes::RPM => String::from("rpm")
        }
    }
}


impl Folder {

    pub fn build(self, target: TargetTypes, packagename: String, outpath: String) -> Option<i32> {
        self.lang.build(self.path, target, packagename, outpath)
    }
}