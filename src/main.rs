use pm_lib::cli::commands;
use pm_lib::application::password_manager;

use clap::Command;
use env_logger;
use log::{info, error};

fn main() {
    env_logger::init();

    let matches = commands::build_cli().get_matches();

    match matches.subcommand() {
        Some(("store", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name")
                .expect("Argument 'name' is required")
                .as_str();
            let value = sub_matches.get_one::<String>("value")
                .expect("Argument 'value' is required")
                .as_str();
            match password_manager::store_password(name, value) {
                Ok(_) => println!("Password stored securely."),
                Err(e) => println!("Failed to store password: {}", e),
            }
        },
        Some(("retrieve", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name")
                .expect("Argument 'name' is required")
                .as_str();
            match password_manager::retrieve_password(name) {
                Ok(password) => println!("Retrieved Password: {}", password),
                Err(e) => println!("Failed to retrieve password: {}", e),
            }
        },
        Some(("delete", sub_matches)) => {
            let name = sub_matches.get_one::<String>("name")
                .expect("Argument 'name' is required")
                .as_str();
            match password_manager::delete_password(name) {
                Ok(_) => println!("Password deleted."),
                Err(e) => println!("Failed to delete password: {}", e),
            }
        },
        Some(("list", _)) => {
            match password_manager::list_passwords() {
                Ok(passwords) => println!("Stored passwords: {:?}", passwords),
                Err(e) => println!("Failed to list passwords: {}", e),
            }
        },
        _ => println!("Invalid command. Use --help for usage."),
    }
}
