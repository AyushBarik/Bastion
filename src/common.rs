extern crate aes_gcm;
extern crate dotenv;
extern crate argon2;
extern crate rand;
extern crate rusqlite;

// use aes_gcm::aead::{Aead, KeyInit, OsRng};
// use aes_gcm::{Aes256Gcm, Key, Nonce}; // AES-GCM with 256-bit key
// use rand::RngCore;

use dotenv::from_path;
use std::env;

use std::process::Command;

use argon2::{
    password_hash::{SaltString, PasswordHasher},
    Argon2,
};

use rusqlite::Result;

fn delicious_juniper() -> String {
    let output = Command::new("/Users/ayush/Desktop/Rust-ML/Bastion/src/Juniper") 
        .output()
        .expect("Failed to execute Juniper");

    String::from_utf8_lossy(&output.stdout).to_string()
}

pub fn hashpass(mut password: String) -> Result<(String, bool), argon2::password_hash::Error> {
    //dotenv().expect("Failed to load .env file");
    let env_path = "/Users/ayush/Desktop/Rust-ML/Bastion/src/.env";
    from_path(env_path).expect("Failed to load .env file");

    println!("{:#?}", password);
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


    //println!("obtained hash: {:#?}", _horn);
    if _horn == password_hash {
        pass = true;
    }
    //println!("Password hash: {}", password_hash);

    Ok((password_hash, pass))
}


pub fn encrypt_password(password: String) -> Result<Vec<u8>> {

    //convert string into binary 
    
    //"password12345@ -> 110100010101110..."

    Ok(vec![12,38]) 


}

pub fn decrypt_password(password: String) -> Result<String> {

    //obtain from database
    //hexstring =....
    //hexstring_to_bin(hex_string)

    //USE MASTERKEY AS ENCRYPTION KEY
    //TRY PEPPERS

    Ok("dummy".to_string()) 
    
}

fn main(){

    println!("Doesn't do anything, just a helper file");
    let m = hashpass("ayush123@".to_string()).unwrap();
    println!("hash: {:#?} bool: {:#?}", m.0, m.1);
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




