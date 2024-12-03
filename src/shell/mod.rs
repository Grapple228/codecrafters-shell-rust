mod error;

pub use error::{Error, Result};
use tracing::debug;

use std::{
    env,
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

fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::ConfigMissingEnv(name))
}

fn _get_env_parse<T: FromStr>(name: &'static str) -> Result<T> {
    let val = get_env(name)?;

    val.parse::<T>().map_err(|_| Error::ConfigWrongFormat(name))
}

fn get_path() -> Result<Vec<String>> {
    let path = get_env("PATH")?;

    Ok(path.split(':').map(|path| path.to_string()).collect())
}

fn path_to_parts(path: &str) -> Vec<String> {
    path.split('/').map(|s| s.to_string()).collect()
}

impl Shell {
    pub fn default() -> Result<Shell> {
        let mut shell = Self {
            stdout: io::stdout(),
            stdin: io::stdin(),
            path_parts: Vec::new(),
        };

        let parts = env::current_dir()?.to_string_lossy().to_string();

        shell.path_parts = path_to_parts(&parts);

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

        let parts = input.trim().split(' ').collect::<Vec<_>>();

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
                        let home = get_env("HOME")?;

                        self.path_parts = path_to_parts(&home);
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
            self.path_parts = path_to_parts(&path);
            path
        };

        env::set_current_dir(path.clone()).map_err(|_| Error::CdProblem(path.to_string()))?;

        Ok(())
    }

    fn pwd(&mut self) -> Result<String> {
        Ok(self.current_dir())
    }

    fn execute(&mut self, command: &str, args: &[&str]) -> Result<()> {
        for path in get_path()? {
            if let Ok(true) = fs::exists(path.clone()) {
                let path = format!("{}/{}", path, command);

                let path = PathBuf::from(path);
                if path.is_file() {
                    match std::process::Command::new(path)
                        .args(args)
                        .stdout(Stdio::piped())
                        .spawn()
                    {
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
        for path in get_path()? {
            let path = format!("{}/{}", path, value);

            if let Ok(true) = fs::exists(path.clone()) {
                return Ok(format!("{} is {}", value, path));
            }
        }

        Err(Error::TypeNotFound(value.to_string()))
    }
}
