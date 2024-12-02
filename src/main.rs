use std::io::{self, Write};

use shell::{Error, Result};
use tracing::debug;
use tracing_subscriber::field::debug;

pub fn report(message: impl Into<String>) {
    eprintln!("Error: {}", message.into());
}

fn main() -> Result<()> {
    shell::init()?;

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("$ ");
        stdout.flush()?;

        // Wait for user input
        let mut input = String::new();
        stdin.read_line(&mut input)?;

        match input.trim() {
            "exit" => {
                return Ok(());
            }
            input => {
                println!("{}: command not found", input);
            }
        }
    }

    Ok(())
}
