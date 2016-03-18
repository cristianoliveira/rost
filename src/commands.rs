use std::io::prelude::*;
use std::fs::File;

pub trait Command {
  fn execute(&self, file: File) -> bool;
}

#[derive(Debug)]
pub struct AddCommand {
    pub args: Vec<String>,
}

impl Command for AddCommand {
    fn execute(&self, mut file: File) -> bool {
       let ip = self.args[2].to_string();
       let host = self.args[3].to_string();
       let new_item = format!("{ip}  {host}", ip=ip, host=host);
       let bytes = match file.write(new_item.as_ref()) {
           Ok(b) => b,
           Err(err) => panic!("file error: {}", err)
       };
       return true;
    }
}

pub struct DeleteCommand {
    pub args: Vec<String>,
}

impl Command for DeleteCommand {
    fn execute(&self, file: File) -> bool {
      println!("Command not implemented");
      return true;
    }
}

pub fn execute(command: String, args: Vec<String>) -> bool {
    let mut file = match File::open("/etc/hosts") {
        Ok(f) => f,
        Err(err) => panic!("file error: {}", err)
    };

    match command.as_ref() {
        "add" => { AddCommand{ args: args }.execute(file); },
        "delete" => { DeleteCommand{ args: args }.execute(file); },
         _  => return false
    };
    return true;
}
