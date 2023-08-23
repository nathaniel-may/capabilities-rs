/// Definition of all the capabilities this application needs to run
use crate::error::Result;
use std::path::Path;

pub trait FileRead {
    fn read(&self, path: &Path) -> Result<String>;
}

pub trait FileWrite {
    fn write(&self, path: &Path, content: &str) -> Result<()>;
}

pub trait Env {
    fn env(&self, key: &str) -> Result<String>;
}

// more commonly a logging capability
pub trait Print {
    fn print(&self, s: &str);
    fn println(&self, s: &str);
}
