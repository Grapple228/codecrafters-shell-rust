#![allow(unused)] // For beginning only.

use std::{
    io::{self, stderr, Write},
    process,
};

use shell::{Result, Shell};
use tracing::debug;

fn main() -> Result<()> {
    shell::init()?;

    let mut shell = Shell::default();

    loop {
        shell.init()?;

        match shell.process_input() {
            Ok(_) => (),
            Err(error) => shell::report(error),
        }
    }
}
