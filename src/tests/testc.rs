//! module that defines the test capabilites
use crate::capabilities::*;
use crate::error::Error::*;
use crate::error::Result;
use std::path::Path;

// contains any necessary test state.
// this is often different caches for state to replace the outside world.
// different tests can have different test capabilities- you can make as
// many of these as you need to test different states (e.g. - db table
// with no data vs with data, an API that responds with a rate limit etc.)
pub struct DefaultTest {
    pub logs: Vec<String>,
    pub counter_file: String,
}

impl DefaultTest {
    pub fn default() -> DefaultTest {
        DefaultTest {
            logs: vec![],
            counter_file: "0".to_string(),
        }
    }
}

impl ReadFiles for DefaultTest {
    fn read_file(&mut self, path: &Path) -> Result<String> {
        let filename = path.file_name().unwrap().to_str().unwrap();
        if filename == "count.txt" {
            Ok(self.counter_file.clone())
        } else {
            Ok("Default file contents".to_string())
        }
    }
}

impl WriteFiles for DefaultTest {
    fn write_file(&mut self, _: &Path, content: &str) -> Result<()> {
        self.counter_file = content.to_string();
        Ok(())
    }
}

impl Env for DefaultTest {
    fn env(&mut self, key: &str) -> Result<String> {
        if key == "READ_FILE" {
            Ok("read.txt".to_string())
        } else if key == "COUNT_FILE" {
            Ok("count.txt".to_string())
        } else {
            Err(MissingEnvVar {
                source: None,
                key: key.to_string(),
            })
        }
    }
}

impl Logging for DefaultTest {
    fn log(&mut self, s: &str) {
        self.logs.push(s.to_string())
    }
}
