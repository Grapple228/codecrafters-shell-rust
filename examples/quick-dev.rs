#![allow(unused)] // For beginning only.

use std::{
    io::{self, stderr, Write},
    process,
};

use shell::{run, Result};
use tracing::debug;

fn main() -> Result<()> {
    run()
}
