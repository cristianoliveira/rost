use std::fs::{File,OpenOptions};
use std::io::prelude::*;

use std::env;
mod commands;

fn die_showing_help() -> ! {
  println!("Rost a command line host manager.

  Usage:
    add [ip] [host] to the host file.
    delete [ip/host] from the host file.
  ");

  std::process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file = match OpenOptions::new().append(true)
                                           .read(true)
                                           .write(true)
                                           .open("/etc/hosts_bkp") {
        Ok(f) => f,
        Err(err) => panic!("file error: {}", err)
    };

    let command = args[1].to_string();
    let execution = match commands::make(command, args) {
      Some(e) => e ,
      None => { die_showing_help(); },
    };

    match execution.execute(file) {
      Ok(_) => { println!("Host changed with success.")},
      Err(err) => panic!("Error while execution: {}", err),
    }
}
