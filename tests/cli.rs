extern crate rost;

use std::fs::{OpenOptions, remove_file, File};
use rost::cli::args::Args;
use rost::cli::command;
use rost::cli::commands::*;
use std::io::prelude::*;
use std::io;

#[test]
fn it_returns_none() {
    let args = Args::new();
    assert!(command(args).is_none());
}

#[test]
fn it_returns_some() {
    let mut args = Args::new();
    args.cmd_add = true;
    args.arg_ip = Some("123".to_string());
    args.arg_host = Some("host".to_string());
    assert!(command(args).is_some());

    let mut args = Args::new();
    args.cmd_del = true;
    args.arg_pattern = Some("123".to_string());
    assert!(command(args).is_some());

    let mut args = Args::new();
    args.cmd_list = true;
    assert!(command(args).is_some());
}

#[warn(unused_must_use)]
#[test]
fn it_add_host_to_the_file(){
    let mut content = String::new();
    let mut file = open_file("test_add.txt");
    AddCommand{
        ip: "123.1.1.1".to_string(),
        host: "test".to_string(),
    }.execute(file);

    file = open_file("test_add.txt");
    file.read_to_string(&mut content);

    assert!(content.contains("test"));
    assert!(content.contains("123.1.1.1"));

    remove_file("test_add.txt");
}

#[warn(unused_must_use)]
#[test]
fn it_delete_host_from_file(){
    let filename = "test_delete.txt";
    // prepare file with some content
    let mut file = open_file(filename);
    file.write(b"123.1.1.1  hostlocal\n");
    file.write(b"123.1.1.2  hostlocal2\n");

    let mut content = String::new();
    let mut file = open_file(filename);
    DeleteCommand{
       pattern: "123.1.1.1".to_string(),
    }.execute(file);

    file = open_file(filename);
    file.read_to_string(&mut content);

    assert!(!content.contains("123.1.1.1"));
    assert!(content.contains("123.1.1.2"));
    remove_file(filename);
}

fn open_file(path: &str) -> File {
    let file = match OpenOptions::new().append(true)
                                       .read(true)
                                       .write(true)
                                       .create(true)
                                       .open(path) {
        Ok(f) => f,
        Err(err) => panic!("Error while openning file: {}", err)
    };
    file
}
