
use std::path::PathBuf;
use std::env::current_dir;
use downloader::DownloadFile;
use builder::TargetTypes;
use parser::{Item, Items};

pub enum ExecutionError{
    NOTVALIDDIR
}

pub struct Batch {
    pub files: Vec<Item>,
    pub to: TargetTypes
}

pub fn convert_absolute(path: String) -> PathBuf{
    let buf :PathBuf = PathBuf::from(&path[..]);
    let cwd :PathBuf = current_dir().unwrap();
    let current_path = buf.as_path();
    cwd.join(current_path)
}


impl Batch{

    pub fn new(filename: String, source: TargetTypes) -> Self{
        let absolute_path :PathBuf = convert_absolute(filename);
        let parsed : Items = Items::new(absolute_path.to_str().unwrap());
        Batch{
            files: parsed.packages,
            to: source
        }
    }

    pub fn download(&self, outpath: String) -> Result<(), ExecutionError>{
        let absolute_path :PathBuf = convert_absolute(outpath);
        if !absolute_path.is_dir(){
            return Err(ExecutionError::NOTVALIDDIR);
        }
        for item in self.files.iter() {
            let tar_name = format!("{}.tar.gz", item.name);
            let tar_path :PathBuf = absolute_path.as_path().join(tar_name);
            let loader = DownloadFile::new(&item.url[..], tar_path.to_str().unwrap());
            loader.start();
        }
        Ok(())
    }
}