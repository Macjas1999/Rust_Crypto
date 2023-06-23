extern crate crypto;
extern crate rand;
use std::fs::File;
use crypto::symmetriccipher::{BlockEncryptor, BlockDecryptor};
use crypto::buffer::{ReadBuffer, WriteBuffer, BufferResult};
use crypto::aes::{cbc_encryptor, cbc_decryptor, KeySize};
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use aes::cipher::consts;
use crypto::{blockmodes::NoPadding, buffer::RefWriteBuffer};
use crypto::buffer::RefReadBuffer;
//use crypto::digest::Digest;
//use crypto::sha2::Sha256;

use std::io::{Read, BufRead};

use rand::Rng;

pub struct InputFile {
    path: String,
    contents: Vec<u8>,
}
//Generator key and nonce generator
pub struct KeyAndNonce {
    // key: Vec<u8>,
    // nonce: Vec<u8>,
    key: [u8; 16],
    nonce: [u8; 16],
}

impl KeyAndNonce {
    // fn generate_key(&mut self) {
    //     let mut gen = rand::thread_rng();
    //     let key: Vec<u8> = (0..16).map(|_| gen.gen_range(33..=126) as u8).collect();
    //     self.key = key.clone();
    // }    
    // fn generate_nonce(&mut self) {
    //     let mut gen = rand::thread_rng();
    //     let nonce: Vec<u8> = (0..16).map(|_| gen.gen_range(33..=126) as u8).collect();
    //     self.nonce = nonce.clone();
    // }
    fn generate_key(&mut self) {
        let mut gen = rand::thread_rng();
        let key: [u8; 16] = (0..16).map(|_| gen.gen_range(33..=126) as u8).collect::<Vec<_>>().try_into().unwrap();
        self.key = key.clone();
    }   
    fn generate_nonce(&mut self) {
        let mut gen = rand::thread_rng();
        let nonce: [u8; 16] = (0..16).map(|_| gen.gen_range(33..=126) as u8).collect::<Vec<_>>().try_into().unwrap();
        self.nonce = nonce.clone();
    } 
}

fn main() {
    let in_path: String = String::from("/home/maciej/Desktop/Rust/Crypto/Rust_Crypto/test.txt");
    let file = InputFile {
        path: in_path.clone(),
        contents: contents_of_file(in_path),
    };

    let mut key_nonce = KeyAndNonce {
        key: [0; 16],
        nonce: [0; 16],
    };
    key_nonce.generate_key();
    key_nonce.generate_nonce();

    println!("\n");
    println!("Directory: {}", file.path);
    for byte in file.contents.clone() {
        print!("{} ", byte);
    }
    println!();

    for byte in file.contents.clone() {
        let singlechar = char::from(byte);
        print!("{}", singlechar);
    }
    print!("Key: ");
    for byte in key_nonce.key.clone() {
        let singlechar = char::from(byte);
        print!("{}", singlechar);
    }
    print!("\n");
    print!("Nonce: ");
    for byte in key_nonce.nonce.clone() {
        let singlechar = char::from(byte);
        print!("{}", singlechar);
    }
    print!("\n");

    let encrypted_data = encryp_handle(file.contents.clone(), &key_nonce.key, &key_nonce.key);
    println!("Encrypted: ");
    for byte in encrypted_data.clone() {
        let singlechar = char::from(byte);
        print!("{}", singlechar);
    }
    print!("\n\n");
    let decrypted_data = decrypt_handle(encrypted_data.clone(), &key_nonce.key, &key_nonce.key);
    println!("Decrypted: ");
    for byte in decrypted_data.clone() {
        let singlechar = char::from(byte);
        print!("{}", singlechar);
    }
    print!("\n")
}
//Functions
///File handling
fn contents_of_file(in_path: String) -> Vec<u8> {
    let mut file = match File::open(in_path) {
        Ok(file) => file,
        Err(error) => {
            let errstr = String::from("Error reading file");
            eprint!("Error reading file: {}", error);
            let vec: Vec<u8> = errstr.into_bytes();
            return vec
        }
    };
    let mut contents: Vec<u8> = Vec::new();
    match file.read_to_end(&mut contents) {
        Ok(_) => return contents,
        Err(_) => {
            
            eprint!("Failed to read the file");
            let errstr = String::from("Failed to read the file");
            let vec: Vec<u8> = errstr.into_bytes();
            return vec
        }
    }  
}

fn encryp_handle(contents: Vec<u8>, key: &[u8; 16], nonce: &[u8; 16]) -> Vec<u8> {
    //Cipher
    let mut encryptor = cbc_encryptor(
        KeySize::KeySize128,
        key,
        nonce,
        crypto::blockmodes::PkcsPadding,
    );

    let mut buffer = [0; 4096];
    let mut result = Vec::<u8>::new();
    
    let mut read_buffer = crypto::buffer::RefReadBuffer::new(&contents); //or use .into_boxed_slice()
    let mut write_buffer = crypto::buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let r = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true)
            .expect("encryption error");

        result.extend(write_buffer.take_read_buffer().take_remaining().iter().copied());

        match r {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    result

}

fn decrypt_handle(contents: Vec<u8>, key: &[u8; 16], nonce: &[u8; 16]) -> Vec<u8> {
    let mut decryptor = cbc_decryptor(
        KeySize::KeySize128,
        key,
        nonce,
        crypto::blockmodes::PkcsPadding,
    );

    let mut buffer = [0; 4096];
    let mut result = Vec::<u8>::new();
    
    let mut read_buffer = crypto::buffer::RefReadBuffer::new(&contents); //or use .into_boxed_slice()
    let mut write_buffer = crypto::buffer::RefWriteBuffer::new(&mut buffer);

    loop {
        let r = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true)
            .expect("decryption error");

        result.extend(write_buffer.take_read_buffer().take_remaining().iter().copied());

        match r {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }
    result
}