mod capabilities;
mod error;
pub mod prodc;
#[cfg(test)]
mod tests;
use crate::capabilities::*;
use crate::error::Error::*;
use crate::error::Result;
use std::path::Path;

/// top-level entry point to the program.
/// Both main and tests can call here for the full application.
pub fn run<W: Env + ReadFiles + WriteFiles + Logging>(world: &mut W) -> Result<()> {
    let read_path = world.env("READ_FILE")?;
    let count_path = world.env("COUNT_FILE")?;
    world.log(&format!("reading from: {}", read_path));
    let contents = world.read_file(Path::new(&read_path))?;
    world.log(&format!("file contents: {}", contents));
    update_counter(world, Path::new(&count_path))?;
    Ok(())
}

/// this function doesn't need all the capabilites of the full program.
/// it's easy to tell what this function does not do.
pub fn update_counter<W: ReadFiles + WriteFiles>(world: &mut W, counter_path: &Path) -> Result<()> {
    let contents: String = world.read_file(counter_path)?;
    let count = contents.parse::<usize>().map_err(|e| BadCounterFile {
        source: Some(e),
        contents: contents,
    })?;
    world.write_file(counter_path, &format!("{}", count + 1))?;
    Ok(())
}

// #[cfg(test)]
// mod tests {
//     mod testc;
//     use testc::DefaultTest;

//     #[test]
//     fn boop() {
//         let mut world = DefaultTest::default();
//         super::run(&mut world);

//         assert_eq!(world.logs, vec!["one", "two"])
//     }
// }
