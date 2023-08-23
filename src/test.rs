/// module that defines the test capabilites
use crate::capabilities::*;
use crate::error::Error::*;
use crate::error::Result;
use std::path::Path;

// contains any necessary test state
// this is often different caches for state to replace the outside world.
// different tests can have different test capabilities- you can make as
// many of these as you need to test different states (e.g. - db table with no data vs with data etc.)
pub struct DefaultTest {
    logs: Vec<String>,
    counter_file: String,
}

impl ReadFiles for DefaultTest {
    fn read_file(&self, path: &Path) -> Result<String> {
        Ok("I return the same thing every time".to_string())
    }
}

impl WriteFiles for DefaultTest {
    fn write_file(&mut self, path: &Path, content: &str) -> Result<()> {
        self.counter_file = content.to_string();
        Ok(())
    }
}

impl Env for DefaultTest {
    fn env(&self, key: &str) -> Result<String> {
        if key == "READ_FILE" {
            Ok("dummy_read_file_for_tests.txt".to_string())
        } else if key == "WRITE_FILE" {
            Ok("dummy_write_file_for_tests.txt".to_string())
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
