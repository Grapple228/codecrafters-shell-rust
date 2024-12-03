use std::io::{self, Write};

use shell::{Error, Result, Shell};
use tracing::debug;
use tracing_subscriber::field::debug;

fn main() -> Result<()> {
    shell::init()?;

    let mut shell = Shell::default()?;

    loop {
        shell.init()?;

        match shell.process_input() {
            Ok(()) => {}
            Err(error) => shell::report(error),
        }
    }
}
