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

    let command = args[1].to_string();
    if commands::execute(command, args) {
        println!("Command executed.");
    } else {
        die_showing_help();
    }
}
