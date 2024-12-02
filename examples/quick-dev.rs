#![allow(unused)] // For beginning only.

use std::{
    io::{self, stderr, Write},
    process,
};

use tracing::debug;

type Error = Box<dyn std::error::Error>;
type Result<T> = core::result::Result<T, Error>; // For tests.

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
