mod capabilities;
mod error;
mod prodc;
#[cfg(test)]
mod test;

use crate::capabilities::*;
use crate::error::Result;
use crate::prodc::Prod;
use std::path::Path;
use std::process::ExitCode;

/// main creates the value that represents the world
fn main() -> ExitCode {
    let prod_world = Prod {};
    match run(prod_world) {
        Err(e) => {
            eprintln!("{}", e);
            ExitCode::FAILURE
        }
        Ok(()) => ExitCode::SUCCESS,
    }
}

/// top-level entry point to the program.
/// Both main and tests can call here for the full application.
fn run<W: Env + ReadFiles + WriteFiles + Logging>(world: W) -> Result<()> {
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
fn update_counter<W: ReadFiles + WriteFiles>(world: W, counter_path: &Path) -> Result<()> {
    let count: usize = world.read_file(counter_path)?.parse().unwrap();
    world.write_file(counter_path, &format!("{}", count + 1))?;
    Ok(())
}
