use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::io::SeekFrom;
use std::io;

pub trait Execution {
  fn execute(&self, file: File) -> io::Result<()>;
}

/// # AddCommand
///
/// It add new host [EOF] using the following format:
/// [ip] [host]
///
/// # Example:
///   127.0.0.0  localhost
///
pub struct AddCommand {
    pub ip: String,
    pub host: String,
}
impl Execution for AddCommand {
    fn execute(&self, mut file: File) -> io::Result<()> {
       let new_item = format!("\n {ip}  {host} #r", ip=self.ip, host=self.host);
       try!(file.write(new_item.as_ref()));
       Ok(())
    }
}

/// # DeleteCommand
///
/// It removes one or more hosts from the file 
/// based on [host_ip] passed as param
///
pub struct DeleteCommand {
    pub host_ip: String,
}
impl Execution for DeleteCommand {
    fn execute(&self, mut file: File) -> io::Result<()> {
      let mut content = String::new();
      file.read_to_string(&mut content);

      let mut new_content = String::new();
      for line in content.lines() {
        if !line.contains(&self.host_ip){
          new_content.push_str(line);
          new_content.push_str("\n");
        }
      };

      file.set_len(0);
      file.write_all(new_content.as_ref())
    }
}

/// # Return a command based on [command] passed as param
/// or None if any command found.
///
/// return Execution
pub fn make(command: String, args: Vec<String>) -> Option<Box<Execution+'static>>{
    let cmd: Box<Execution> = match command.as_ref() {
        "add" => Box::new(AddCommand{
            ip: args[2].to_string(),
            host: args[3].to_string(),
        }),
        "delete" => Box::new(DeleteCommand{
            host_ip: args[2].to_string(),
        }),
         _  => return None
    };
    return Some(cmd);
}
