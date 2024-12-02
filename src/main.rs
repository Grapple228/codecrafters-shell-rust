use std::io::{self, Write};

use shell::{Error, Result};

pub fn report(message: impl Into<String>) {
    eprintln!("Error: {}", message.into());
}

fn main() -> Result<()> {
    print!("$ ");
    io::stdout().flush()?;

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();

    stdin.read_line(&mut input)?;

    match input.trim() {
        input => {
            println!("{}: command not found", input);
            return Err(Error::UnknownCommand(input.to_string()));
        }
    }

    Ok(())
}
