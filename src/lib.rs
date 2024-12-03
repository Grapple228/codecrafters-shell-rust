// region:    --- Modules

use std::{
    fmt::format,
    io::{self, Write},
    process,
};

use tracing::{debug, info};
use tracing_subscriber::EnvFilter;

// -- Modules
mod config;
mod error;
mod shell;

// -- Flatten
pub use config::config;
pub use error::{Error, Result};
pub use shell::Shell;

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
    _ = config();

    Ok(())
}

pub fn report(error: shell::Error) {
    match error {
        shell::Error::CommandNotFound(command) => {
            println!("{}: command not found", command);
        }
        shell::Error::TypeNotFound(value) => println!("{}: not found", value),
        shell::Error::Io(error) => todo!(),
        shell::Error::ParseIntError(parse_int_error) => todo!(),
    }
}
