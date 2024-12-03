mod env;
mod error;

pub use error::{Error, Result};
use tracing::debug;
use tracing_subscriber::field::debug;

use std::{
    fmt::format,
    fs,
    io::{self, Write},
    os::unix::process::CommandExt,
    path::{self, PathBuf},
    process::{self, Stdio},
    str::FromStr,
};

#[derive(Debug)]
pub struct Shell {
    stdout: io::Stdout,
    stdin: io::Stdin,
    path_parts: Vec<String>,
}

fn split_path(path: &str) -> Vec<String> {
    path.split('/').map(|s| s.to_string()).collect()
}

fn split_input(input: &str) -> Vec<String> {
    let mut parts = Vec::new();

    let mut is_quoted = false;
    let mut current = String::new();

    for (i, c) in input.chars().enumerate() {
        match c {
            '\'' => {
                if is_quoted {
                    parts.push(current);
                    current = String::new();
                }
                is_quoted = !is_quoted;
            }
            ' ' => {
                if is_quoted {
                    current.push(c);
                } else {
                    if !current.is_empty() {
                        parts.push(current);
                        current = String::new();
                    }
                }
            }
            other => {
                current.push(other);
            }
        }
    }

    if !current.is_empty() {
        parts.push(current);
    }

    parts
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
        let mut input = String::new();
        self.stdin.read_line(&mut input)?;

        let splitted = split_input(&input.trim());

        let parts = splitted.iter().map(|s| s.as_str()).collect::<Vec<_>>();

        match parts.as_slice() {
            [""] => {}
            ["exit", code] => Shell::exit(code.parse()?),
            ["type", value] => println!("{}", self.type_info(value)?),
            ["echo", ..] => {
                let message = parts[1..].join(" ");
                println!("{}", message);
            }
            ["cd", path] => self.cd(path)?,
            ["pwd"] => println!("{}", self.pwd()?),
            [input, ..] => {
                return self.execute(input, &parts[1..]);
            }
            _ => {
                unreachable!();
            }
        }

        Ok(())
    }

    fn cd(&mut self, path: &str) -> Result<()> {
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

    fn pwd(&mut self) -> Result<String> {
        Ok(self.current_dir())
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
                                debug!("here");
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

    fn exit(code: i32) -> ! {
        process::exit(code)
    }

    fn echo(&mut self, message: impl Into<String>) {
        println!("{}", message.into());
    }

    fn type_info(&mut self, value: &str) -> Result<String> {
        const BUILTINS: [&str; 4] = ["echo", "type", "exit", "pwd"];

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

// region:    --- Tests

#[cfg(test)]
mod tests {
    type Error = Box<dyn std::error::Error>;
    type Result<T> = core::result::Result<T, Error>; // For tests.

    use super::*;

    #[test]
    fn test_split_input_ok() -> Result<()> {
        let input = "";

        assert_eq!(split_input(input), Vec::<String>::new());

        let input = "''";

        assert_eq!(split_input(input), vec![""]);

        let input = "a b c";

        assert_eq!(split_input(input), vec!["a", "b", "c"]);

        let input = "'a b c'";

        assert_eq!(split_input(input), vec!["a b c"]);

        let input = "a 'b c'";

        assert_eq!(split_input(input), vec!["a", "b c"]);

        let input = "a      b";

        assert_eq!(split_input(input), vec!["a", "b"]);

        let input = "'a      b'";

        assert_eq!(split_input(input), vec!["a      b"]);

        Ok(())
    }
}

// endregion: --- Tests
