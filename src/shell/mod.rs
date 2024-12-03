mod error;

pub use error::{Error, Result};

use std::{
    fs,
    io::{self, Write},
    os::unix::process::CommandExt,
    path::{self, PathBuf},
    process,
};

use crate::config;

#[derive(Debug)]
pub struct Shell {
    stdout: io::Stdout,
    stdin: io::Stdin,
}

impl Shell {
    pub fn default() -> Shell {
        Shell {
            stdout: io::stdout(),
            stdin: io::stdin(),
        }
    }

    pub fn init(&mut self) -> Result<()> {
        print!("$ ");
        self.stdout.flush()?;

        Ok(())
    }

    pub fn process_input(&mut self) -> Result<()> {
        let mut input = String::new();
        self.stdin.read_line(&mut input)?;

        let parts = input.trim().split(' ').collect::<Vec<_>>();

        match parts.as_slice() {
            [""] => {}
            ["exit", code] => Shell::exit(code.parse()?),
            ["type", value] => println!("{}", self.type_info(value)?),
            ["echo", ..] => {
                let message = parts[1..].join(" ");
                println!("{}", message);
            }
            [input, ..] => {
                return self.execute(input, &parts[1..]);
            }
            _ => {
                unreachable!();
            }
        }

        Ok(())
    }

    fn execute(&mut self, command: &str, args: &[&str]) -> Result<()> {
        for path in Self::get_path() {
            if let Ok(true) = fs::exists(path.clone()) {
                let path = format!("{}/{}", path, command);

                let path = PathBuf::from(path);
                if path.is_file() {
                    match std::process::Command::new(path).args(args).spawn() {
                        Ok(c) => match c.wait_with_output() {
                            Ok(output) => {
                                print!("{}", String::from_utf8_lossy(&output.stdout));
                                return Ok(());
                            }
                            Err(_) => break,
                        },
                        Err(_) => break,
                    };
                };
            }
        }

        Err(Error::CommandNotFound(command.to_string()))
    }

    fn get_path() -> Vec<String> {
        config()
            .path
            .split(':')
            .map(|path| path.to_string())
            .collect()
    }

    fn exit(code: i32) -> ! {
        process::exit(code)
    }

    fn echo(&mut self, message: impl Into<String>) {
        println!("{}", message.into());
    }

    fn type_info(&mut self, value: &str) -> Result<String> {
        match value {
            "echo" | "type" | "exit" => Ok(format!("{} is a shell builtin", value)),
            _ => self.system_type_info(value),
        }
    }

    fn system_type_info(&mut self, value: &str) -> Result<String> {
        for path in Self::get_path() {
            let path = format!("{}/{}", path, value);

            if let Ok(true) = fs::exists(path.clone()) {
                return Ok(format!("{} is {}", value, path));
            }
        }

        Err(Error::TypeNotFound(value.to_string()))
    }
}
