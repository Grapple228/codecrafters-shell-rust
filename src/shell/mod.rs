mod error;

pub use error::{Error, Result};

use std::{
    fs,
    io::{self, Write},
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
                return Err(Error::CommandNotFound(input.to_string()));
            }
            _ => {
                unreachable!();
            }
        }

        Ok(())
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
        let paths = &config()
            .path
            .split(':')
            .map(|path| path.to_string())
            .collect::<Vec<String>>();

        for path in paths {
            let path = format!("{}/{}", path, value);

            if let Ok(true) = fs::exists(path.clone()) {
                return Ok(format!("{} is {}", value, path));
            }
        }

        Err(Error::TypeNotFound(value.to_string()))
    }
}
