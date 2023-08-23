//! main creates the value that represents the world
use app::prodc::Prod;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut prod_world = Prod {};
    match app::run(&mut prod_world) {
        Err(e) => {
            eprintln!("{}", e);
            ExitCode::FAILURE
        }
        Ok(()) => ExitCode::SUCCESS,
    }
}
