#![allow(unused)] // For beginning only.

use std::{
    io::{self, stderr, Write},
    process,
};

type Error = Box<dyn std::error::Error>;
type Result<T> = core::result::Result<T, Error>; // For tests.

fn main() -> Result<()> {
    print!("$ ");
    io::stdout().flush()?;

    // Wait for user input
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input)?;

    Ok(())
}
