// region:    --- Modules

use tracing_subscriber::EnvFilter;

// -- Modules
mod error;
mod extensions;
mod shell;
mod splitter;

// -- Flatten
pub use error::{Error, Result};
pub use extensions::{CharExt, StringExt};
pub use shell::Shell;
pub use splitter::Splitter;

// endregion: --- Modules

pub struct W<T>(T);

pub fn init() -> Result<()> {
    // LOGGING INITIALIZATION
    tracing_subscriber::fmt()
        .without_time() // For early development
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}

pub fn report(error: shell::Error) {
    match error {
        shell::Error::CommandNotFound(command) => {
            println!("{}: command not found", command);
        }
        shell::Error::ExecuteProblem(message) => println!("{}: execute problem", message),
        shell::Error::CdProblem(path) => {
            println!("cd: {}: No such file or directory", path)
        }
        shell::Error::InvalidArgumentsCount {
            command,
            expected,
            actual,
        } => {
            println!(
                "{}: invalid number of arguments: expected {}, got {}",
                command, expected, actual
            )
        }
        shell::Error::TypeNotFound(value) => println!("{}: not found", value),
        shell::Error::Io(error) => println!("{}", error),
        shell::Error::ParseIntError(error) => println!("{}", error),
        shell::Error::ConfigMissingEnv(name) => println!("{}: missing environment variable", name),
        shell::Error::ConfigWrongFormat(name) => {
            println!("{}: wrong format of environment variable", name)
        }
    }
}
