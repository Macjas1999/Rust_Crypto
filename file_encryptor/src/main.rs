extern crate crypto;
extern crate rand;
//use crypto::digest::Digest;
//use crypto::sha2::Sha256;
use crypto::aes::KeySize::KeySize128;
use crypto::symmetriccipher::SynchronousStreamCipher;

use rand::Rng;
//use rand::rngs::OsRng;
use std::fs;

pub struct InputFile {
    path: String,
    contents: String,
}

impl InputFile {
    
}

fn main() {
    let in_path: String = String::from("/home/maciej/Desktop/Rust/Crypto/Rust_Crypto/test.txt");
    let file = InputFile {
        path: in_path.clone(),
        contents: contents_of_file(in_path),
    };
    println!("\n");
    println!("Directory: {}", file.path);
    println!("Contents: {}", file.contents);
    println!("\n");
    println!("{}", encryp_handle(file.contents));
    println!("\n");
}
//Functions
///File handling
fn contents_of_file(in_path: String) -> String {
    match fs::read_to_string(in_path) {
        Ok(contents) => {
            return contents;
            //println!("File contents:\n{}", contents);
        }
        Err(error) => {
            eprintln!("Error reading file: {}", error);
            return String::from("Error reading file");
        }
    }
}
fn encryp_handle(contents: String) -> String {

    //Generator key and nonce generator
    let mut gen = rand::thread_rng();//.gen_range(33..=126) as u8 as char;
    let key: Vec<u8> = (0..16).map(|_| gen.gen_range(33..=126) as u8).collect();
    let nonce: Vec<u8> = (0..16).map(|_| gen.gen_range(33..=126) as u8).collect();
    //Cipher
    let mut cipher = crypto::aes::ctr(KeySize128, key.as_slice(), nonce.as_slice());
    //Processing files
    let s_input: Vec<u8> = contents.as_bytes().to_vec();
    let mut s_output: Vec<u8> = Vec::with_capacity(s_input.len());
  
    cipher.process(&s_input, &mut s_output);
    let e_output = String::from_utf8_lossy(&s_output);
    let e_output: String = e_output.to_string(); 
    e_output
}