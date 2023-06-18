extern crate rust_crypto;

use std::fs

pub struct InputFile {
    path: String,
    contents: String,
}

impl InputFile {
    
}

fn main() {
    readFile(String::from("/home/maciej/Desktop/Rust/Crypto/Rust_Crypto/test.txt"));
    println!("{}", file.contents);
}
//Functions
///File handling
fn readFile(in_path: String) {
    pub let mut file = InputFile {
        path: in_path,
        contents: contentsOfFile(in_path),
    }
}
fn contentsOfFile(in_path: Strin) -> String {
    match fs::read_to_string("path/to/file.txt") {
        Ok(contents) => {
            return contents;
            //println!("File contents:\n{}", contents);
        }
        Err(error) => {
            eprintln!("Error reading file: {}", error);
        }
    }
}