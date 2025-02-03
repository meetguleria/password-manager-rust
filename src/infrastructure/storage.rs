use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;
use log::{info, error};

lazy_static! {
    static ref PASSWORD_STORE: Mutex<HashMap<String, Vec<u8>>> = Mutex::new(HashMap::new());
}

pub fn store(name: String, encrypted_value: Vec<u8>) -> Result<(), String> {
    let mut store = PASSWORD_STORE.lock().map_err(|e| format!("Failed to lock storage: {}", e))?;
    store.insert(name.clone(), encrypted_value);
    info!("Password '{}' stored successfully.", name);
    Ok(())
}

pub fn retrieve(name: &str) -> Result<Option<Vec<u8>>, String> {
    let store = PASSWORD_STORE.lock().map_err(|e| format!("Failed to lock storage: {}", e))?;
    let result = store.get(name).cloned();
    if result.is_some() {
        info!("Password '{}' retrieved successfully.", name);
    } else {
        error!("Password '{}' not found.", name);
    }
    Ok(result)
}

pub fn delete(name: &str) -> Result<(), String> {
    let mut store = PASSWORD_STORE.lock().map_err(|e| format!("Failed to lock storage: {}", e))?;
    store.remove(name);
    info!("Password '{}' deleted successfully.", name);
    Ok(())
}

pub fn list() -> Result<Vec<String>, String> {
    let store = PASSWORD_STORE.lock().map_err(|e| format!("Failed to lock storage: {}", e))?;
    let passwords: Vec<String> = store.keys().cloned().collect();
    info!("Listed {} passwords.", passwords.len());
    Ok(passwords)
}
