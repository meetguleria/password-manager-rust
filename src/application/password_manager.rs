use crate::usecases::password_manager;

pub fn store_password(name: &str, value: &str) -> Result<(), String> {
    password_manager::store_password(name, value)
}

pub fn retrieve_password(name: &str) -> Result<String, String> {
    password_manager::retrieve_password(name)
}

pub fn delete_password(name: &str) -> Result<(), String> {
    password_manager::delete_password(name)
}

pub fn list_passwords() -> Result<Vec<String>, String> {
    password_manager::list_passwords()
}
