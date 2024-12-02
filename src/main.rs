use std::io::{self, Write};

use shell::{Error, Result};
use tracing::debug;
use tracing_subscriber::field::debug;

pub fn report(message: impl Into<String>) {
    eprintln!("Error: {}", message.into());
}

fn main() -> Result<()> {
    shell::run()
}
