extern crate crypto;
extern crate rand;
use crypto::{blockmodes::NoPadding, buffer::RefWriteBuffer};
use crypto::buffer::RefReadBuffer;
//use crypto::digest::Digest;
//use crypto::sha2::Sha256;
use crypto::symmetriccipher::SynchronousStreamCipher;
use crypto::aes::KeySize::KeySize128;

use std::io::{Read, BufRead};
//use crypto::aes::ctr::CtrMode;
use crypto::symmetriccipher::BlockEncryptor;
use crypto::buffer::{BufferResult, WriteBuffer};
// use aes::Aes128;
// use block_modes::BlockMode;
// use block_modes::block_padding::Pkcs7;
// use block_modes::ctr::Ctr128;
// use hex_literal::hex;

use rand::Rng;
//use rand::rngs::OsRng;
use std::fs;

pub struct InputFile {
    path: String,
    contents: String,
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
    println!("Contents: {}", file.contents);
    println!("\n");
    let mess: String = encryp_handle(file.contents, key_nonce.key.clone(), key_nonce.nonce.clone());
    println!("{}", mess);
    println!("\n");
    println!("{}", decrypt_handle(mess, key_nonce.key.clone(), key_nonce.nonce.clone()));
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
fn encryp_handle(contents: String, key: [u8; 16], nonce: [u8; 16]) -> String {
    //Cipher
    //let mut cipher = crypto::aes::ctr(KeySize128, key.as_slice(), nonce.as_slice());
    let mut cipher = crypto::aes::cbc_encryptor(KeySize128, &key, &nonce, NoPadding); 
    //let mut cipher = CtrMode::new(KeySize128, &key, &nonce);
    //Processing files
    //let s_input: Vec<u8> = contents.as_bytes().to_vec();
    //let mut s_output: Vec<u8> = s_input.clone();
    let input_b = contents.as_bytes();
    let input_c = std::io::Cursor::new(input_b);
    let mut input_buff = RefReadBuffer::new(input_c.get_ref());
    //let mut s_output: Vec<u8> = contents.as_bytes().to_vec();
    //let mut s_output: Vec<u8> = Vec::with_capacity(s_input.len());
    let mut output_b: Vec<u8> = Vec::new();
    let mut output_c = std::io::Cursor::new(&mut output_b);
    let mut output_buff = RefWriteBuffer::new(output_c.get_mut());
    cipher.encrypt(&mut input_buff, &mut output_buff, false).unwrap();
    // let output_s = match out_a {
    //     Ok(BufferResult::Buffer(buffer)) => {
    //         str::from_utf8(buffer).unwrap().to_string()
    //     }
    // };
    //cipher.process(&s_input, &mut s_output);
    //let e_output = String::from_utf8_lossy(&s_output);
    //let e_output: String = e_output.to_string(); 
    
    let e_output = String::from_utf8_lossy(&output_b);
    let e_output: String = e_output.to_string();
    e_output

}

fn decrypt_handle(contents: String, key: [u8; 16], nonce: [u8; 16]) -> String {

   let mut decipher = crypto::aes::cbc_decryptor(KeySize128, &key, &nonce, NoPadding);

   let input_b = contents.as_bytes();
   let input_c = std::io::Cursor::new(input_b);
   let mut input_buff = RefReadBuffer::new(input_c.get_ref());

   let mut output_b: Vec<u8> = Vec::new();
   let mut output_c = std::io::Cursor::new(&mut output_b);
   let mut output_buff = RefWriteBuffer::new(output_c.get_mut());

   decipher.decrypt(&mut input_buff, &mut output_buff, false).unwrap();

   let e_output = String::from_utf8_lossy(&output_b);
   let e_output: String = e_output.to_string();
   e_output
}


//the right one


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