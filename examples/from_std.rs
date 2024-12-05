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

    let mut shell = Shell::default()?;

    loop {
        match shell.process_input() {
            Ok(_) => (),
            Err(error) => shell::report(error),
        }
    }

    Ok(())
}
