//! module that defines the production capabilites
use crate::capabilities::*;
use crate::error::Error::*;
use crate::error::Result;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

// contains any necessary application state
// this is often application configurations like a global log level
pub struct Prod {}

impl ReadFiles for Prod {
    fn read_file(&mut self, path: &Path) -> Result<String> {
        std::fs::read_to_string(path).map_err(|e| FileIOFailed { source: Some(e) })
    }
}

impl WriteFiles for Prod {
    fn write_file(&mut self, path: &Path, content: &str) -> Result<()> {
        let mut file = File::create(path).map_err(|e| FileIOFailed { source: Some(e) })?;
        file.write_all(content.as_bytes())
            .map_err(|e| FileIOFailed { source: Some(e) })?;
        Ok(())
    }
}

impl Env for Prod {
    fn env(&mut self, key: &str) -> Result<String> {
        std::env::var(key).map_err(|e| MissingEnvVar {
            source: Some(e),
            key: key.to_string(),
        })
    }
}

impl Logging for Prod {
    // you would use a real logging library here
    fn log(&mut self, s: &str) {
        println!("{}", s)
    }
}
