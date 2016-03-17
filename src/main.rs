use std::env;
use commands::Command;
mod commands;

fn die_showing_help() -> ! {
  println!("Rost a command line host manager.

  Usage:
    add [ip] [host] to the host file.
  ");

  std::process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = args[0].to_string();
    let cli_command = commands::build(command, args);
    let cmd = match cli_command {
       Some(cli_command) => cli_command,
       None => { die_showing_help(); },
    };
    cmd.execute();
}
