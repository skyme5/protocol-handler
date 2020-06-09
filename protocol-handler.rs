#![windows_subsystem = "windows"]

use std::env;
use std::fs;
use std::process::Command;

use std::io;
use std::io::prelude::*;

use json;
use url::percent_encoding::percent_decode;

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn main() {
    let protocols = fs::read_to_string("E:/projects/personal/protocol-handler/protocol-handlers.json")
                    .expect("Something went wrong reading the file");
    pause();
    let parsed = json::parse(&protocols).unwrap();
    let args: Vec<String> = env::args().collect();

    println!("{}", args[1]);

    let uri: Vec<&str> = args[1].split("://").collect();
    let filepath = percent_decode(uri[1].replace("/", "\\").as_bytes()).decode_utf8().unwrap().to_string();
    let handler: Vec<&str> = filepath.split(".").collect();
    let program = parsed[handler.last().unwrap().to_lowercase().to_string()].to_string();
    println!("{} {}", handler.last().unwrap().to_lowercase().to_string(), program);

    Command::new(program)
            .arg(filepath)
            .status()
            .expect("failed to execute process");
}
