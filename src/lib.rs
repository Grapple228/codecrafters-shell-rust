// region:    --- Modules

use std::{
    io::{self, Write},
    process,
};

use tracing::info;
use tracing_subscriber::EnvFilter;

// -- Modules
mod config;
mod error;

// -- Flatten
pub use config::config;
pub use error::{Error, Result};

// endregion: --- Modules

pub struct W<T>(T);

pub fn init() -> Result<()> {
    // LOGGING INITIALIZATION
    tracing_subscriber::fmt()
        .without_time() // For early development
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("Initializing");

    // CONFIG INITIALIZATION
    info!("Loading config...");
    let _ = config();

    Ok(())
}

pub fn run() -> Result<()> {
    init()?;

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("$ ");
        stdout.flush()?;

        // Wait for user input
        let mut input = String::new();
        stdin.read_line(&mut input)?;

        match input.trim().split(' ').collect::<Vec<_>>()[..] {
            ["exit", code] => {
                process::exit(code.parse()?);
                return Ok(());
            }
            ["echo", message] => {
                println!("{}", message);
            }
            [input, ..] => {
                println!("{}: command not found", input);
            }
            _ => {
                unreachable!();
            }
        }
    }
}
