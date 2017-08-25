
use std::io;
use std::io::{Read, Write, BufWriter};
use std::fs::File;
use reqwest;
use reqwest::{Client, Response};
use reqwest::header::ContentLength;
use pbr::ProgressBar;

pub enum DownloadStatus {
    COMPLETED,
    ERRORED,
    INTERUPTED
}

#[derive(Clone, Copy, Debug)]
pub struct DownloadFile<'a> {
    pub url: &'a str,
    pub path: &'a str
}

pub fn tar_filename(url: &str) -> String {
    let url_string = String::from(url);
    let fragment = url_string.split("/").last().unwrap();
    String::from(fragment)
}


fn download_file(url: &str, path: &str)  {
    let mut response : Response = send_request(url).unwrap();
    let cont_length: u64 = response.headers().get::<ContentLength>()
        .map(|ct_len| **ct_len)
        .unwrap_or(0);
    let _  = File::create(path)
        .and_then(|out_file| {
            let mut writer = BufWriter::new(out_file);
            download_tar(cont_length, &mut response, &mut writer)
        });

}

fn send_request(url: &str) -> Result<Response, reqwest::Error>{
    let rest_client: Client = Client::builder().unwrap()
                                     .gzip(true)
                                     .build()?;
    rest_client.get(url)?.send()
}

fn download_tar<R, W>(cn: u64, reader: &mut R, writer: &mut W) -> Result<DownloadStatus, io::Error>
        where R: Read, W: Write {
    let mut pbar = ProgressBar::new(cn);
    pbar.format("╢▌▌░╟");
    let mut buf = [0; 8192];
    loop {
        let read_bytes = match reader.read(&mut buf) {
            Ok(0) => return {
                pbar.finish_print("Download Complete!");
                Ok(DownloadStatus::COMPLETED)
            },
            Ok(len) => len,
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Ok(DownloadStatus::ERRORED)
        };
        let _ = writer.write_all(&buf[..read_bytes]);
        pbar.add(read_bytes as u64);
    }
}


impl<'a> DownloadFile<'a> {

    pub fn new(url: &'a str, path: &'a str) -> Self {
        DownloadFile{
            url: url,
            path: path
        }
    }

    pub fn start(&self){
        download_file(self.url, self.path)
    }

}