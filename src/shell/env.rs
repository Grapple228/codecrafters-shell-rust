use std::{env, str::FromStr};

use crate::shell::{Error, Result};

pub fn get_env(name: &'static str) -> Result<String> {
    env::var(name).map_err(|_| Error::ConfigMissingEnv(name))
}

pub fn _get_env_parse<T: FromStr>(name: &'static str) -> Result<T> {
    let val = get_env(name)?;

    val.parse::<T>().map_err(|_| Error::ConfigWrongFormat(name))
}

pub fn get_path() -> Result<Vec<String>> {
    let path = get_env("PATH")?;

    Ok(path.split(':').map(|path| path.to_string()).collect())
}

pub fn set_current_dir(path: &str) -> Result<()> {
    env::set_current_dir(path).map_err(|_| Error::CdProblem(path.to_string()))
}

pub fn current_dir() -> Result<String> {
    Ok(env::current_dir()?.to_string_lossy().to_string())
}
