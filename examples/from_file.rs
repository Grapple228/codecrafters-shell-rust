#![allow(unused)] // For beginning only.

use std::{
    fs,
    io::{self, stderr, Write},
    process,
};

use shell::{Result, Shell, Splitter};
use tracing::{debug, info};

fn main() -> Result<()> {
    shell::init()?;

    let command = fs::read_to_string("command.txt")?;
    let command = command.trim();
    debug!("command: '{}'", command);

    let mut shell = Shell::default()?;

    match shell.process_command(&command) {
        Ok(_) => (),
        Err(error) => shell::report(error),
    }
    Ok(())
}
