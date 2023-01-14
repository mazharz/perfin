use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn write_to_file(path: String, content: String) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&path)
        .unwrap_or_else(|file| panic!("Couldn't open file: {}\n{}.", path, file));

    writeln!(file, "{}", content)
        .unwrap_or_else(|path| panic!("Couldn't write to file: {}.", path));
}
