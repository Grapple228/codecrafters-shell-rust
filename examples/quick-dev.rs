#![allow(unused)] // For beginning only.

use std::{io::stderr, process};

type Error = Box<dyn std::error::Error>;
type Result<T> = core::result::Result<T, Error>; // For tests.

fn main() -> Result<()> {
    shell::init();

    Ok(())
}
