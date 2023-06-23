extern crate crypto;
extern crate rand;
use crypto::buffer::{ReadBuffer, WriteBuffer, BufferResult, self};
use crypto::aes::{cbc_encryptor, cbc_decryptor, KeySize};

use std::fs::File;
use std::io::{self, Read};
use std::convert::TryInto;

use rand::Rng;

pub struct InputFile {
    path: String,
    contents: Vec<u8>,
}
//Generator key and nonce generator
pub struct KeyAndNonce {
    key: [u8; 16],
    nonce: [u8; 16],
}

impl KeyAndNonce {
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
    loop {
        println!("Do you want to encrypt a file or decrypt [e/d]");
        println!("To exit input [o]");
        let input_char: char = read_char();
        
        if input_char == 'e' {
            println!("Have you got a file to encrypt or you want to ecrypt a message right from console input? [f/c]");
            let input_char: char = read_char();

            if input_char == 'f' {
                let in_path = read_string();
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
                
                let encrypted_data = encrypt_handle(file.contents.clone(), &key_nonce.key, &key_nonce.nonce);
                println!("Encrypted: ");
                for byte in encrypted_data.clone() {
                    let singlechar = char::from(byte);
                    print!("{}", singlechar);
                }
                print!("\n\n");

                println!("Exit / back to menu [o/m]");
                let input_char = read_char();
                if input_char == 'o' {
                    break;
                }
                else if input_char == 'm' {
                    continue;
                }
                        
            } else if input_char == 'c' {
                println!("Write here your message");
                let input_message = read_string();
                let input_message: Vec<u8> = input_message.into_bytes();
                
                let mut key_nonce = KeyAndNonce {
                    key: [0; 16],
                    nonce: [0; 16],
                };
                key_nonce.generate_key();
                key_nonce.generate_nonce();
                
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
                
                let encrypted_data = encrypt_handle(input_message.clone(), &key_nonce.key, &key_nonce.key);
                println!("Encrypted: ");
                print!("|");
                for byte in encrypted_data.clone() {
                    let singlechar = char::from(byte);
                    print!("{}", singlechar);
                }
                print!("|");
                print!("\n\n");

                println!("Exit / back to menu [o/m]");
                let input_char = read_char();
                if input_char == 'o' {
                    break;
                }
                else if input_char == 'm' {
                    continue;
                }
                
            } else {
                println!("Invalid input");
                continue;
            }
            //let in_path: String = String::from("/home/maciej/Desktop/Rust/Crypto/Rust_Crypto/test.txt");
            
        } else if input_char == 'd' {
            //println!("Write here your message to decrypt");
            //let input_message: Vec<u8> = read_string().into_bytes();
            //let input_message = read_encrypted();
            println!("Provide a path to a file");
            let in_path = read_string();
            let file = InputFile {
                path: in_path.clone(),
                contents: contents_of_file(in_path),
            };

            let mut key_nonce = KeyAndNonce {
                key: [0; 16],
                nonce: [0; 16],
            };
            println!("\nSupply key");
            key_nonce.key = read_keyandnonce(read_string());
            println!("\nSupply nonce");
            key_nonce.nonce = read_keyandnonce(read_string());
            print!("\n\n");
            let decrypted_data = decrypt_handle(file.contents.clone(), &key_nonce.key, &key_nonce.key);
            println!("Decrypted: ");
            for byte in decrypted_data.clone() {
                let singlechar = char::from(byte);
                print!("{}", singlechar);
            }
            print!("\n");

            println!("Exit / back to menu [o/m]");
            let input_char = read_char();
            
            if input_char == 'o' {
                break;
            } else if input_char == 'm' {
                continue;
            }
            
        } else if input_char == 'o' {
            break;
        } else {
            print!("Invalid input");
            continue;
        }
    }
}
//Functions
//Read char
fn read_char() -> char {
    let mut in_buffer: String = String::new();
    match io::stdin().read_line(&mut in_buffer) {
        Ok(_) => {
            if let Some(prob_char) = in_buffer.trim().chars().next() {
                let input_char = prob_char;
                return input_char;
            } else {
                return '0';
            }
        }
        Err(error) => {
            eprintln!("Failed to read input: {}", error);
            return '0';
        }
    }
}
//Read String
fn read_string() -> String {
    let mut input = String::new();
                        
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input = input.trim().to_string();
            return input;
        }
        Err(error) => {
            eprintln!("Failed to read input: {}", error);
            return error.to_string();
        }
    }
}
fn read_encrypted() -> Vec<u8> {       
    let mut contents: Vec<u8> = Vec::new();

    match io::stdin().read_to_end(&mut contents) {
        Ok(_) => return contents,
        Err(_) => {
            eprint!("Failed to read the file");
            let errstr = String::from("Failed to read the file");
            let vec: Vec<u8> = errstr.into_bytes();
            return vec
        }
    }  
}
fn read_keyandnonce(input: String) -> [u8; 16] {
    let byte_slice: &[u8] = input.as_bytes();
    let array: [u8; 16] = byte_slice.try_into().unwrap();
    array
}
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

//Cipher
fn encrypt_handle(contents: Vec<u8>, key: &[u8; 16], nonce: &[u8; 16]) -> Vec<u8> {    let mut encryptor = cbc_encryptor(
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
    //let data = contents.as_slice();
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
    let padding_length = crypto::blockmodes::pkcs_padding::pad_with_length(result.len(), 16);
    result.extend(vec![padding_length as u8; padding_length]);

    result
}