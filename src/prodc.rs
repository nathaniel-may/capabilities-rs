/// module that defines the production capabilites
use crate::capabilities::*;
use crate::error::Error::*;
use crate::error::Result;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct Prod {}

impl FileRead for Prod {
    fn read(&self, path: &Path) -> Result<String> {
        std::fs::read_to_string(path).map_err(|e| FileIOFailed { source: Some(e) })
    }
}

impl FileWrite for Prod {
    fn write(&self, path: &Path, content: &str) -> Result<()> {
        let mut file = File::create(path).map_err(|e| FileIOFailed { source: Some(e) })?;
        file.write_all(content.as_bytes())
            .map_err(|e| FileIOFailed { source: Some(e) })?;
        Ok(())
    }
}

impl Env for Prod {
    fn env(&self, key: &str) -> Result<String> {
        std::env::var(key).map_err(|e| MissingEnvVar {
            source: Some(e),
            key: key.to_string(),
        })
    }
}

impl Print for Prod {
    fn print(&self, s: &str) {
        print!("{}", s)
    }

    fn println(&self, s: &str) {
        println!("{}", s)
    }
}
