use derive_more::derive::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    CommandNotFound(String),
    TypeNotFound(String),

    // -- Externals
    #[from]
    Io(std::io::Error),
    #[from]
    ParseIntError(std::num::ParseIntError),
}

// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate