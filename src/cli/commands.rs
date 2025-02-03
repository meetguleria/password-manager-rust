use clap::{Command, Arg};

pub fn build_cli() -> Command {
    Command::new("Local Password Manager")
        .version("1.0")
        .about("A secure, local password manager")
        .arg(
Arg::new("quiet")
    .short('q')
    .long("quiet")
    .help("Run in quiet mode")
        )
        .subcommand(
            Command::new("store")
                .about("Stores an encrypted password")
                .arg(Arg::new("name").required(true).help("Name of the password"))
                .arg(Arg::new("value").required(true).help("Password value"))
        )
        .subcommand(
            Command::new("retrieve")
                .about("Retrieves a password")
                .arg(Arg::new("name").required(true).help("Name of the password to retrieve"))
        )
        .subcommand(
            Command::new("delete")
                .about("Deletes a stored password")
                .arg(Arg::new("name").required(true).help("Name of the password to delete"))
        )
        .subcommand(
            Command::new("list")
                .about("Lists all stored passwords")
        )
}
