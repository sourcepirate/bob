
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

extern crate serde;
extern crate serde_json;
extern crate reqwest;
extern crate pbr;
extern crate tar;
extern crate flate2;
extern crate glob;
extern crate clap;



mod downloader;
mod parser;
mod unarchiver;
mod builder;
mod lang;
mod fpm;
mod executor;


use std::env::current_dir;
use downloader::DownloadFile;
use lang::python::Python;
use lang::ruby::Ruby;
use builder::{Packable, Folder, TargetTypes};
use unarchiver::Unarchiver;
use parser::Items;
use executor::Batch;
use clap::{Arg, App, ArgMatches};
use clap::AppSettings;
use executor::convert_absolute;



fn define_target(s: &str) -> TargetTypes {
    match s {
        "rpm" => TargetTypes::RPM,
        "deb" => TargetTypes::DEB,
        _ => TargetTypes::RPM
    }
}


fn main(){
    let cwp = current_dir().unwrap();
    let cwd = cwp.to_str().unwrap();
    let app = App::new("Bob the builder")
        .version("0.5")
        .author("sourcepirate")
        .about("Build rpms and debs with ease")
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::with_name("config")
            .short("f")
            .long("config")
            .value_name("config.json")
            .help("Config file for building rpm")
            .takes_value(true))
        .arg(Arg::with_name("bin")
            .short("b")
            .long("binary")
            .value_name("rpm")
            .help("Type of binary to build")
            .takes_value(true))
        .arg(Arg::with_name("target")
            .short("o")
            .long("out")
            .value_name(cwd)
            .help("target directory for rpms")
            .takes_value(true))
        .arg(Arg::with_name("tmp")
            .short("t")
            .long("temp")
            .value_name("/tmp")
            .help("Temporary directory for rpms")
            .takes_value(true));

    let matches: ArgMatches = app.get_matches();

    let file_name = matches.value_of("config").unwrap_or("config.json");
    let bin_type = define_target(matches.value_of("bin").unwrap_or("rpm"));
    let out_path = matches.value_of("target").unwrap_or(cwd.clone());
    let temp_path = matches.value_of("tmp").unwrap_or("/tmp");


    let mut batch = Batch::new(file_name.to_string(),
                               bin_type,
                               temp_path.to_string(),
                               out_path.to_string());

    batch.download_and_build();


}

