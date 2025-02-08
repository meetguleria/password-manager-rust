use std::io::{self, Write};
use crate::application::password_manager;

pub fn start_interactive_mode() {
  println!("Password Manager Interactive Mode (type 'exit' to quit)");
  loop {
    print!("> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    if input == "exit" {
      break;
    }
    
    let args: Vec<&str> = input.split_whitespace().collect();
    let result = password_manager::parse_command(input);
    match result {
      Ok(output) => {
        println!("{}", output);
      },
      Err(error) => {
        println!("Error: {}", error);
      }
    }
  }
}