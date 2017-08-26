
use std::io;
use std::io::{Read};
use std::fs::{File, ReadDir};
use std::path::Path;
use tar::Archive;
use flate2::read::GzDecoder;

pub struct Unarchiver{
    pub path: String
}

impl Unarchiver{
    pub fn new(path: &str) -> Self {
        Unarchiver{
            path: path.to_string()
        }
    }

    pub fn unarchive(&self, dest: String) -> io::Result<()>{
        let fp = Path::new(&self.path[..]);
        let mut files = fp.read_dir().unwrap();
        for file in files{
            let file_path = file.unwrap().path();
            let file_name = file_path.to_str().unwrap();
            if file_name.contains(".tar"){
                let opened_file : File= File::open(file_name).unwrap();
                let decompressed = GzDecoder::new(opened_file).unwrap();
                let mut archive = Archive::new(decompressed);
                archive.unpack(&dest[..]).expect("Unable to unpack");
            }
        }
        Ok(())
    }
}