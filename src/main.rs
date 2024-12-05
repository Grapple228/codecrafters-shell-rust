use shell::Shell;

use shell::Result;

fn main() -> Result<()> {
    shell::init()?;

    let mut shell = Shell::default()?;

    loop {
        match shell.process_input() {
            Ok(()) => {}
            Err(error) => shell::report(error),
        }
    }
}
