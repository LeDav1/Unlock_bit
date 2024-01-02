use std::error::Error;
use std::fmt;

use aes_gcm::aead::{Aead, generic_array::GenericArray, OsRng, AeadCore, KeyInit};
use aes_gcm::Aes256Gcm;

use hex::decode;

use std::fs::File;
use std::io::{Read, Write};

#[derive(Debug)]
struct CustomError(aes_gcm::Error);

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AES-GCM error: {:?}", self.0)
    }
}

impl Error for CustomError {}

//encrypts any file with aes in counter mode
pub fn encrypt_aes(file_path: &str) -> Result<(), Box<dyn std::error::Error>>
{
    //random values
    //let key = Aes256Gcm::generate_key(OsRng);
    //let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    //known values : to change later
    let key = "9311d97f2dd4e73fea8e60d3e1130c4cdc611e47261e498e2de6647bc477d1f7";
    let nonce = "301a35dbfce88096d1fbf0b7";

    println!("Key used : {:#}", key);
    println!("Nonce used : {:#}", nonce);
    
    // Read file content
    let mut input_file = File::open(file_path)?;
    let mut input_data = Vec::new();
    input_file.read_to_end(&mut input_data)?;

    // Create AES-GCM encryptor
    let cipher = Aes256Gcm::new(GenericArray::from_slice(key.as_bytes()));

    // Encrypt file content
    let ciphertext = cipher.encrypt(GenericArray::from_slice(nonce.as_bytes()), input_data.as_slice()).map_err(|e| CustomError(e))?;
    //let ciphertext = cipher.encrypt(GenericArray::from_slice(nonce), input_data.as_slice())?;

    // Write encrypted content to the output file
    let mut output_file = File::create(file_path)?;
    output_file.write_all(&ciphertext)?;

    Ok(())
}

//decrypts any file with aes in counter mode
//requires the key and nonce used during encryption
pub fn decrypt_aes(file_path: &str, key_hex: &str, nonce_hex: &str) -> Result<(), Box<dyn std::error::Error>>
{
    let key = decode(key_hex)?;
    let nonce = decode(nonce_hex)?;
    
    // Read file content
    let mut encrypted_file = File::open(file_path)?;
    let mut encrypted_data = Vec::new();
    encrypted_file.read_to_end(&mut encrypted_data)?;

    // Create AES-GCM encryptor
    let cipher = Aes256Gcm::new(GenericArray::from_slice(&key));

    // Decrypt file content
    let ciphertext = cipher.decrypt(GenericArray::from_slice(&nonce), encrypted_data.as_slice()).map_err(|e| CustomError(e))?;

    // Decrypt encrypted content to the output file
    let mut output_file = File::create(file_path)?;
    output_file.write_all(&ciphertext)?;

    Ok(())
}