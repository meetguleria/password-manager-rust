#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::application::password_manager;

    #[test]
    fn test_store_and_retrieve_password() {
        let name = "test_password";
        let value = "test_value";

        // Store password
        assert!(password_manager::store_password(name, value).is_ok());

        // Retrieve password
        let retrieved_password = password_manager::retrieve_password(name);
        assert!(retrieved_password.is_ok());
        assert_eq!(retrieved_password.unwrap(), value);

        // Delete password
        assert!(password_manager::delete_password(name).is_ok());

        // Ensure password is deleted
        let retrieved_password = password_manager::retrieve_password(name);
        assert!(retrieved_password.is_err());
    }

    #[test]
    fn test_list_passwords() {
        let name1 = "test_password1";
        let value1 = "test_value1";
        let name2 = "test_password2";
        let value2 = "test_value2";

        // Store passwords
        assert!(password_manager::store_password(name1, value1).is_ok());
        assert!(password_manager::store_password(name2, value2).is_ok());

        // List passwords
        let passwords = password_manager::list_passwords();
        assert!(passwords.is_ok());
        let passwords = passwords.unwrap();
        assert_eq!(passwords.len(), 2);
        assert!(passwords.contains(&name1.to_string()));
        assert!(passwords.contains(&name2.to_string()));

        // Delete passwords
        assert!(password_manager::delete_password(name1).is_ok());
        assert!(password_manager::delete_password(name2).is_ok());
    }

    #[test]
    fn test_delete_nonexistent_password() {
        let name = "nonexistent_password";

        // Attempt to delete nonexistent password
        let result = password_manager::delete_password(name);
        assert!(result.is_ok());
    }
}
