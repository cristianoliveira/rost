mod cli;

extern crate rustc_serialize;
extern crate docopt;

use std::fs::{OpenOptions};
use std::io::prelude::*;
use docopt::Docopt;
use cli::args::Args;

const USAGE: &'static str = "
Rost a command line host manager.
** May require sudo to change hosts file

Usage:
  rost add <ip> <host>
  rost del <pattern>
  rost list
  rost -h
  rost -v

Options:
  -h --help     Show this screen.
";


fn die_showing_help() -> ! {
  println!("{}", USAGE);
  std::process::exit(1);
}

fn main() {
    let args: Args= Docopt::new(USAGE)
                     .and_then(|dopt| dopt.decode())
                     .unwrap_or_else(|e| e.exit());

    let execution = match cli::command(args) {
       Some(e) => e,
       None => { die_showing_help(); },
    };

    let file = match OpenOptions::new().append(true)
                                       .read(true)
                                       .write(true)
                                       .open("/etc/hosts") {
        Ok(f) => f,
        Err(err) => panic!("Error while openning file: {}", err)
    };

    match execution.execute(file) {
      Ok(_) => { println!("Command executed.") },
      Err(err) => panic!("Error while execution: {}", err),
    }
}
