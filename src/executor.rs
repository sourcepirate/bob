
use std::path::PathBuf;
use std::fs::{read_dir, ReadDir};
use std::env::current_dir;
use downloader::DownloadFile;
use builder::{TargetTypes, SourceTypes};
use unarchiver::Unarchiver;
use builder::{Packable, Folder};
use lang::ruby::Ruby;
use lang::python::Python;
use parser::{Item, Items};

#[derive(Debug, Clone)]
pub enum ExecutionError{
    NOTVALIDDIR,
    BATCHFAILURE,
    SOURCENOTFOUND
}

#[derive(Debug, Clone)]
pub struct Batch {
    pub files: Vec<Item>,
    pub to: TargetTypes,
    pub outpath: String,
    pub rpmpath: String
}

pub fn convert_absolute(path: String) -> PathBuf{
    let buf :PathBuf = PathBuf::from(&path[..]);
    let cwd :PathBuf = current_dir().unwrap();
    let current_path = buf.as_path();
    cwd.join(current_path)
}

pub fn join(p1: &str, p2: &str) -> PathBuf{
    let parent : PathBuf = PathBuf::from(p1);
    let fragment : PathBuf = PathBuf::from(p2);
    parent.join(fragment)
}


pub fn select_language(lang: &str) -> Option<SourceTypes>{
    match lang {
        "python" => Some(SourceTypes::PYTHON),
        "ruby" => Some(SourceTypes::GEM),
        _ => None
    }
}

pub fn select_struct(lang: SourceTypes) -> Option<Box<Packable>>{
    match lang {
        SourceTypes::PYTHON => Some(Box::new(Python)),
        SourceTypes::GEM => Some(Box::new(Ruby)),
        _ => None
    }
}

impl Batch{

    pub fn new(filename: String, source: TargetTypes, outpath: String, rpmpath: String) -> Self{
        let absolute_path :PathBuf = convert_absolute(filename);
        let parsed : Items = Items::new(absolute_path.to_str().unwrap());
        Batch{
            files: parsed.packages,
            to: source,
            outpath: outpath,
            rpmpath: rpmpath
        }
    }

    pub fn download_and_build(&mut self) -> Result<(), ExecutionError>{
        let absolute_path :PathBuf = convert_absolute(self.outpath.clone());
        let absolute_rpm_path: PathBuf = convert_absolute(self.rpmpath.clone());
        if !absolute_path.is_dir(){
            return Err(ExecutionError::NOTVALIDDIR);
        }
        for item in self.files.iter() {
            let tar_name = format!("{}.tar.gz", item.name);
            let tar_path :PathBuf = absolute_path.as_path().join(tar_name);
            let loader = DownloadFile::new(&item.url[..], tar_path.to_str().unwrap());
            loader.start();
            let unarchiver: Unarchiver = Unarchiver::new(&self.outpath[..]);
            let path_zip : String = unarchiver.unarchive(tar_path.to_str().unwrap().to_string(), self.outpath.clone()).unwrap();

            if let Some(source_lang) = select_language(&item.language[..]) {
                let lang_impl  = select_struct(source_lang).unwrap();
                let folder = Folder {
                        path: path_zip,
                        lang: lang_impl
                    };
                folder.build(self.to,
                             item.name.clone(),
                             absolute_rpm_path.to_str().unwrap().to_string());

            } else {
                return Err(ExecutionError::SOURCENOTFOUND);
            }

        }
        Ok(())
    }


}