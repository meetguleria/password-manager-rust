use aes_gcm::{Aes256Gcm, Key, Nonce, aead::{Aead}};
use aes_gcm::aead::KeyInit;
use rand::RngCore;
use log::{info, error};
use std::env;

const KEY_ENV_VAR: &str = "ENCRYPTION_KEY";

pub fn get_key() -> Result<[u8; 32], String> {
    match env::var(KEY_ENV_VAR) {
        Ok(key) => {
            // Convert hex string to bytes
            let mut key_array = [0u8; 32];
            for i in 0..16 {
                let byte = u8::from_str_radix(&key[i*2..i*2+2], 16)
                    .map_err(|e| format!("Invalid hex in key: {}", e))?;
                key_array[i] = byte;
            }
            Ok(key_array)
        }
        Err(e) => Err(format!("Failed to load encryption key: {}", e)),
    }
}

pub fn encrypt(data: &str) -> Result<Vec<u8>, String> {
    let key: [u8; 32] = get_key()?;
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key));
    let mut nonce = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce);
    match cipher.encrypt(&Nonce::from_slice(&nonce), data.as_bytes()) {
        Ok(ciphertext) => {
            info!("Encryption successful.");
            Ok([nonce.to_vec(), ciphertext].concat())
        }
        Err(e) => {
            error!("Encryption failed: {}", e);
            Err(format!("Encryption failed: {}", e))
        }
    }
}

pub fn decrypt(encrypted_data: &[u8]) -> Result<String, String> {
    let key: [u8; 32] = get_key()?;
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key)); // Fixed type inference issue
    let (nonce, ciphertext) = encrypted_data.split_at(12);
    match cipher.decrypt(&Nonce::from_slice(nonce), ciphertext) {
        Ok(plaintext) => {
            info!("Decryption successful.");
            match String::from_utf8(plaintext) {
                Ok(decrypted_value) => Ok(decrypted_value),
                Err(e) => {
                    error!("Invalid UTF-8: {}", e);
                    Err(format!("Invalid UTF-8: {}", e))
                }
            }
        }
        Err(e) => {
            error!("Decryption failed: {}", e);
            Err(format!("Decryption failed: {}", e))
        }
    }
}
