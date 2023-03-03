use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn read_file(path: &str) -> String {
    return fs::read_to_string(path)
        .expect("Should have been able to read the file");
}

pub fn write_to_file(path: &str, content: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .unwrap();

    if let Err(e) = writeln!(file, "{}", content) {
        eprintln!("Couldn't write to file: {}", e);
    }
}
