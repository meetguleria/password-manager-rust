use crate::infrastructure::{encryption, storage};
use log::{info, error};

pub fn store_password(name: &str, value: &str) -> Result<(), String> {
    match encryption::encrypt(value) {
        Ok(encrypted_value) => {
            storage::store(name.to_string(), encrypted_value)?;
            info!("Password '{}' stored successfully.", name);
            Ok(())
        }
        Err(e) => {
            error!("Failed to encrypt password '{}': {}", name, e);
            Err(format!("Failed to encrypt password: {}", e))
        }
    }
}

pub fn retrieve_password(name: &str) -> Result<String, String> {
    match storage::retrieve(name)? {
        Some(encrypted_value) => match encryption::decrypt(&encrypted_value) {
            Ok(decrypted_value) => {
                info!("Password '{}' retrieved successfully.", name);
                Ok(decrypted_value)
            }
            Err(e) => {
                error!("Failed to decrypt password '{}': {}", name, e);
                Err(format!("Failed to decrypt password: {}", e))
            }
        },
        None => {
            error!("Password '{}' not found.", name);
            Err(format!("Password '{}' not found.", name))
        }
    }
}

pub fn delete_password(name: &str) -> Result<(), String> {
    storage::delete(name)?;
    info!("Password '{}' deleted successfully.", name);
    Ok(())
}

pub fn list_passwords() -> Result<Vec<String>, String> {
    let passwords = storage::list()?;
    info!("Listed {} passwords.", passwords.len());
    Ok(passwords)
}
