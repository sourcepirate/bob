
use std::io;
use std::io::{Read, Error};
use std::fs::{File, ReadDir, read_dir};
use std::path::Path;
use std::borrow::Cow;
use tar::{Archive, Entry};
use flate2::read::GzDecoder;
use std::collections::HashMap;
use executor::join;



pub struct Unarchiver{
    pub path: String
}

impl Unarchiver{
    pub fn new(path: &str) -> Self {
        Unarchiver{
            path: path.to_string()
        }
    }

    pub fn unarchive(&self, tar_path: String, dest: String) -> Result<String, Error>{
        let opened_file : File= File::open(&tar_path[..]).unwrap();
        let decompressed = GzDecoder::new(opened_file).unwrap();
        let mut archive = Archive::new(decompressed);
        let mut up: String = String::new();
        let mut first: bool = true;
        for file in archive.entries().unwrap() {
            let mut file = file.unwrap();
            let val = file.header().path().unwrap().to_str().unwrap().to_string();
            let buf = join(&dest[..], &val[..]).to_str().unwrap().to_string();
            if first {
                up = buf;
                first = false;
            }
            file.unpack_in(&dest[..]).unwrap();
        }
        Ok(up)
    }
}