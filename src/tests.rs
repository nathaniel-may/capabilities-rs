mod testc;
use testc::DefaultTest;

#[test]
fn counter_increments() {
    let mut world = DefaultTest::default();
    match super::run(&mut world) {
        Ok(()) => {
            // test that the counter file increments
            // because this doesn't use real files, this test can be run in
            // any order without any cleanup processes.
            assert_eq!(world.counter_file, "1")
        }
        Err(e) => assert_eq!("Ok(())", format!("Err({})", e)),
    }
}

#[test]
fn expect_exact_logs() {
    let mut world = DefaultTest::default();
    match super::run(&mut world) {
        Ok(()) => {
            // test that this is the exact logs that are generated
            // a contrived example- but normally a difficult test to write!
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
