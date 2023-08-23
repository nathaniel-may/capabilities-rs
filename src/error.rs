use std::env;
use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::result::Result as StdResult;
use Error::*;

pub type Result<T> = StdResult<T, Error>;

/// custom error type for the application
#[derive(Debug)]
pub enum Error {
    FileIOFailed {
        source: Option<io::Error>,
    },
    MissingEnvVar {
        source: Option<env::VarError>,
        key: String,
    },
    BadCounterFile {
        source: Option<std::num::ParseIntError>,
        contents: String,
    },
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            FileIOFailed { source } => match source {
                // simplest way to get the types to coerce
                None => None,
                Some(e) => Some(e),
            },
            MissingEnvVar { source, .. } => match source {
                // simplest way to get the types to coerce
                None => None,
                Some(e) => Some(e),
            },
            BadCounterFile { source, .. } => match source {
                // simplest way to get the types to coerce
                None => None,
                Some(e) => Some(e),
            },
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileIOFailed { .. } => write!(f, "File IO failed."),
            MissingEnvVar { key, .. } => write!(f, "No env var named {}", key),
            BadCounterFile { contents, .. } => {
                write!(f, "Counter file is not a number. Contents: {}", contents)
            }
        }
    }
}
