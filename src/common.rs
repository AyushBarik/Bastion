extern crate aes_gcm;
extern crate dotenv;
extern crate argon2;
extern crate rand;
extern crate rusqlite;

use aes_gcm::{
    aead::{consts::U12, Aead, AeadCore, KeyInit, OsRng}, Aes256Gcm, Key, Nonce // Or `Aes128Gcm`
};

use std::error::Error;

use dotenv::from_path;
use std::env;

use std::process::Command;

use argon2::{
    password_hash::{SaltString, PasswordHasher},
    Argon2,
};

use rusqlite::Result;


pub fn hashpass(mut password: String) -> Result<([u8; 32], bool), argon2::password_hash::Error> {
    //dotenv().expect("Failed to load .env file");
    let env_path = "/Users/ayush/Desktop/Rust-ML/Bastion/src/.env";
    from_path(env_path).expect("Failed to load .env file");

    //println!("{:#?}", password);
    password = password.trim_end().to_string();
    // secret salt
    password += &delicious_juniper();

    // static salt
    let salt = SaltString::from_b64("CH67TINExyz12345").expect("Unable to create fixed salt");

    //println!("Using salt: {}", salt);

    let argon2 = Argon2::default();

    // Hash the password
    let password_hash = match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(hash) => {
            println!("Password hashed successfully!");
            println!("{:#?}", hash.to_string());
            hash.to_string()
            
        }
        Err(err) => {
            println!("Error hashing password: {:?}", err);
            return Err(err);
        }
    };

    let mut pass = false;
    let _horn = env::var("OSMANTHUS").expect("HASH NOT STORED .ENV");


    //PASS or FAIL?
    if _horn == password_hash {
        pass = true;
    }
    //derive KEY

    let mut output_key_material: [u8; 32] = [0u8; 32]; 
    Argon2::default().hash_password_into(password.as_bytes(), b"ksjeple_astl", &mut output_key_material)?;

    //println!("AAAA   {:#?}", output_key_material);

    Ok((output_key_material, pass))
}


pub fn encrypt_password(password: &str, raw_key: &[u8; 32]) -> Result<(Vec<u8>, Vec<u8>), Box<dyn Error>> {
    
    let key = Key::<Aes256Gcm>::from_slice(raw_key);
    
    let cipher = Aes256Gcm::new(key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);

    let encrypted_password = cipher
        .encrypt(&nonce, password.as_bytes())
        .map_err(|e| format!("Encryption failed: {:?}", e))?; // Remove `.into()` here

    Ok((encrypted_password, nonce.to_vec()))
}



pub fn decrypt_password(encrypted_password: Vec<u8>, nonce: Vec<u8>, raw_key: &[u8; 32]) -> Result<String> {

    let key = Key::<Aes256Gcm>::from_slice(raw_key);
    let cipher = Aes256Gcm::new(key);
    let nonce_new: &aes_gcm::aead::generic_array::GenericArray<u8, U12> = Nonce::from_slice(&nonce);

    let decrypted_password = cipher.decrypt(nonce_new, encrypted_password.as_ref())
        .map_err(|_| rusqlite::Error::InvalidQuery)?; // Map to rusqlite::Error` BAD PRACTICE CHANGE LATER

    let plaintext = String::from_utf8(decrypted_password)
        .map_err(|_| rusqlite::Error::InvalidQuery)?; // Map to rusqlite::Error` BAD PRACTICE CHANGE LATER


    Ok(plaintext)
    
}

pub fn bin_to_hexstring(binary: Vec<u8>) -> Option<String> {

    let mut hex: String = "x".to_string();
    for byte in binary{

        let add_hex: String = format!("{:x}", byte);
        hex = hex + &add_hex;
    
    }

    hex += "\n";
    
    Some(hex)
}

pub fn hexstring_to_bin(mut hex_string: String) -> Option<Vec<u8>> {

    if hex_string.starts_with('x'){
        hex_string = hex_string[1..].to_string();
    } //remove x from hex i.e. "xAB12" -> "AB12" (IF IT EXISTS)

    let mut bytes: Vec<u8> = vec![]; 

    //DOESNT WORK FOR ODD HEXSTRING (MIGHT FIX - LOW PRIORITY)?
    for i in (0..hex_string.len()).step_by(2){
        if i+2 > hex_string.len() {
            eprintln!("SYSTEM ERROR - Hex string is not well-formed!");
            return None
        }
        let pair = &hex_string[i..i+2];

        bytes.push(u8::from_str_radix(pair, 16).expect("SYSTEM ERROR - COMMON.RS - FAIL HEXSTRING_TO_BIN FN"));
    }

    //println!("Vector: {:#?}", bytes); 

    Some(bytes)
}

fn delicious_juniper() -> String {
    let output = Command::new("/Users/ayush/Desktop/Rust-ML/Bastion/src/Juniper") 
        .output()
        .expect("Failed to execute Juniper");

    String::from_utf8_lossy(&output.stdout).to_string()
}

fn main() {
    println!("Why are you running this?");
}




