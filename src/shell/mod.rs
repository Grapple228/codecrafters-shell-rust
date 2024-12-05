mod env;
mod error;

pub use error::{Error, Result};

use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
    process::{self},
};

use crate::Splitter;

#[derive(Debug)]
pub struct Shell {
    stdout: io::Stdout,
    stdin: io::Stdin,
    path_parts: Vec<String>,
}

fn split_path(path: &str) -> Vec<String> {
    path.split('/').map(|s| s.to_string()).collect()
}

impl Shell {
    pub fn default() -> Result<Shell> {
        let mut shell = Self {
            stdout: io::stdout(),
            stdin: io::stdin(),
            path_parts: Vec::new(),
        };

        shell.path_parts = split_path(&env::current_dir()?);

        Ok(shell)
    }

    pub fn init(&mut self) -> Result<()> {
        print!("$ ");
        self.stdout.flush()?;

        Ok(())
    }

    pub fn current_dir(&self) -> String {
        self.path_parts.join("/")
    }

    pub fn process_input(&mut self) -> Result<()> {
        self.init()?;

        let mut input = String::new();
        self.stdin.read_line(&mut input)?;

        self.process_command(&input)
    }

    pub fn process_command(&mut self, input: &str) -> Result<()> {
        let mut splitter = Splitter::default();

        let splitted = splitter.split(input.trim());

        let parts = splitted.iter().map(|s| s.as_str()).collect::<Vec<_>>();

        // echo "/tmp/foo/f\n8" "/tmp/foo/f\29" "/tmp/foo/f'\'88"
        match parts.as_slice() {
            ["exit", ..] => Shell::exit(&parts[1..]),
            ["type", ..] => println!("{}", self.type_info(&parts[1..])?),
            ["echo", ..] => self.echo(&parts[1..]),
            ["cd", ..] => self.cd(&parts[1..])?,
            ["pwd", ..] => self.pwd()?,
            [input, ..] => {
                return self.execute(input, &parts[1..]);
            }
            _ => {}
        }

        Ok(())
    }

    fn cd(&mut self, args: &[&str]) -> Result<()> {
        let path = args.first();

        if path.is_none() {
            return Ok(());
        }

        let path = path.unwrap();

        let path = if !path.starts_with('/') {
            let nav_parts = path.split('/').collect::<Vec<_>>();

            for part in nav_parts {
                match part {
                    ".." => {
                        self.path_parts.pop();
                    }
                    "." => {
                        // do nothing
                    }
                    "~" => {
                        let home = env::get_env("HOME")?;

                        self.path_parts = split_path(&home);
                    }
                    "" => {}
                    folder => {
                        self.path_parts.push(folder.to_string());
                    }
                }
            }

            self.current_dir()
        } else {
            let path = path.to_string();
            self.path_parts = split_path(&path);
            path
        };

        env::set_current_dir(&path)?;

        Ok(())
    }

    fn pwd(&mut self) -> Result<()> {
        println!("{}", self.current_dir());
        Ok(())
    }

    fn execute(&mut self, command: &str, args: &[&str]) -> Result<()> {
        for path in env::get_path()? {
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
                            Err(_) => {
                                break;
                            }
                        },
                        Err(_) => {
                            break;
                        }
                    };
                };
            }
        }

        Err(Error::CommandNotFound(command.to_string()))
    }

    fn exit(args: &[&str]) -> ! {
        let code = args.first().and_then(|s| s.parse().ok()).unwrap_or(0);

        process::exit(code)
    }

    fn echo(&mut self, args: &[&str]) {
        let message = args.join(" ");
        println!("{}", message);
    }

    fn type_info(&mut self, args: &[&str]) -> Result<String> {
        const BUILTINS: [&str; 5] = ["echo", "cd", "type", "exit", "pwd"];

        let value = args.first().ok_or(Error::InvalidArgumentsCount {
            command: "type".to_string(),
            expected: 1,
            actual: 0,
        })?;

        match value {
            v if BUILTINS.contains(&v) => Ok(format!("{} is a shell builtin", v)),
            _ => self.system_type_info(value),
        }
    }

    fn system_type_info(&mut self, value: &str) -> Result<String> {
        for path in env::get_path()? {
            let path = format!("{}/{}", path, value);

            if let Ok(true) = fs::exists(path.clone()) {
                return Ok(format!("{} is {}", value, path));
            }
        }

        Err(Error::TypeNotFound(value.to_string()))
    }
}
