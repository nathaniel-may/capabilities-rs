/// Definition of all the capabilities this application needs to run
use crate::error::Result;
use std::path::Path;

pub trait ReadFiles {
    fn read_file(&self, path: &Path) -> Result<String>;
}

pub trait WriteFiles {
    fn write_file(&self, path: &Path, content: &str) -> Result<()>;
}

pub trait Env {
    fn env(&self, key: &str) -> Result<String>;
}

pub trait Logging {
    fn log(&self, s: &str);
}

// Other common capabilities to add here for your program:
// - reading system time
// - random number generation
// - database interactions (e.g. - `Database` or `QueryDB` and `InsertDB` )
// - connecting to individual APIs (e.g. - `GetWeather` `Discord` )
