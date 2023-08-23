mod testc;
use crate::capabilities::*;
use std::path::Path;
use testc::DefaultTest;

/// test that the function increments the counter without a full application run.
/// because this doesn't use real files, this test can be run in
/// any order without any cleanup processes.
#[test]
fn counter_increments() {
    let mut world = DefaultTest::default();
    let path_name = world.env("COUNT_FILE").unwrap();
    let path = Path::new(&path_name);
    match super::update_counter(&mut world, path) {
        Ok(()) => assert_eq!(world.counter_file, "1"),
        Err(e) => assert_eq!("Ok(())", format!("Err({})", e)),
    }
}

/// test that the counter file increments after running the full application.
/// because this doesn't use real files, this test can be run in
/// any order without any cleanup processes.
#[test]
fn black_box_counter_increments() {
    let mut world = DefaultTest::default();
    match super::run(&mut world) {
        Ok(()) => assert_eq!(world.counter_file, "1"),
        Err(e) => assert_eq!("Ok(())", format!("Err({})", e)),
    }
}

/// test that this is the exact logs that are generated.
/// a contrived example- but normally a difficult test to write!
#[test]
fn expect_exact_logs() {
    let mut world = DefaultTest::default();
    match super::run(&mut world) {
        Ok(()) => {
            assert_eq!(
                world.logs,
                vec![
                    "reading from: read.txt",
                    "file contents: Default file contents"
                ]
            )
        }
        Err(e) => assert_eq!("Ok(())", format!("Err({})", e)),
    }
}
