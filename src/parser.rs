
use std::fs::File;
use std::io;
use std::io::{Read, BufReader};
use serde_json::{from_str, Value};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub struct Item {
    pub name: String,
    pub url: String,
    pub language: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Items{
   pub packages: Vec<Item>
}


impl Items {

    pub fn new(filename: &str) -> Self {
        let file : File = File::open(filename).unwrap();
        let mut reader = BufReader::new(file);
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer).unwrap();
        let mut items = from_str::<Items>(&buffer[..]).unwrap();
        items
    }
}
