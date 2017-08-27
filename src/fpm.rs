
use std::process::{Child, ExitStatus, Command, Stdio};
use std::io;

pub enum FPMError{
    FILENAMENOTFOUND,
    PACKAGENAMENOTFOUND,
    FAILED
}


pub struct Fpm {
    pub source_type: String,
    pub target_type: String,
    pub file: String,
    pub name: String,
    pub arch: String
}

impl Fpm {

    pub fn new(source: String, target: String) -> Self {
         Fpm {
             source_type: source,
             target_type: target,
             file: String::from(""),
             name: String::from(""),
             arch: String::from("all"),
         }
    }

    pub fn file(&mut self, file_pack: String) -> &Self {
        self.file = file_pack;
        self
    }

    pub fn name(&mut self, name: String) -> &Self {
        self.name = name;
        self
    }

    pub fn arch(&mut self, arc: String) -> &Self {
        self.arch = arc;
        self
    }

    pub fn execute(&self) -> Result<ExitStatus, FPMError> {
        let mut command = Command::new("fpm");
        command.arg("-s");
        command.arg(&self.source_type[..]);
        command.arg("-t");
        command.arg(&self.target_type[..]);
        if self.arch.is_empty(){
            command.arg("-a");
            command.arg(&self.arch[..]);
        }
        if !self.name.is_empty(){
            command.arg("-n");
            command.arg(&self.name[..]);
        } else {
            return Err(FPMError::PACKAGENAMENOTFOUND);
        }

        if !self.file.is_empty(){
            command.arg(&self.file[..]);
            command.stdout(Stdio::inherit());
            let r_status = command.status();
            match r_status {
                Ok(status) => {
                    Ok(status)
                },
                _ => {
                    Err(FPMError::FAILED)
                }
            }
        }
        else {
            Err(FPMError::FILENAMENOTFOUND)
        }
    }
}