mod capabilities;
mod error;
mod prodc;

use crate::capabilities::*;
use crate::error::Result;
use crate::prodc::Prod;
use std::process::ExitCode;

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

fn run<W: Env + Print>(world: W) -> Result<()> {
    let path = world.env("READ_FILE")?;
    world.println(&format!("path: {}", path));
    Ok(())
}
