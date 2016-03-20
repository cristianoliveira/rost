use std::io::prelude::*;
use std::fs::File;
use std::io;

/// # Execution
///
/// Process to be executed in the fle.
///
/// params [file] to be changed.
///
/// return io::Result, may return io errors.
///
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
       let host = format!("\n{ip}  {host} #rost-added", ip=self.ip, host=self.host);
       try!(file.write(host.as_ref()));
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
        try!(file.read_to_string(&mut content));

        let mut new_content = String::new();
        for line in content.lines() {
            if !line.contains(&self.host_ip){
                new_content.push_str(line);
                new_content.push_str("\n");
            }
        };

        try!(file.set_len(0));
        file.write_all(new_content.as_ref())
    }
}

/// # ListCommand
///
/// It lists all current hosts in the file.
///
pub struct ListCommand;
impl Execution for ListCommand {
    fn execute(&self, mut file: File) -> io::Result<()> {
        let mut content = String::new();
        try!(file.read_to_string(&mut content));
        println!("{}", content);
        Ok(())
    }
}

