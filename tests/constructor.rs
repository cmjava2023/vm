use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn constructor() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cmjava")?;

    cmd.arg("tests/data/constructor/Main.class");
    // contains both an assert.failure() and an assert.success(),
    // so that failure() can be simply removed when all features
    // are implemented
    cmd.assert().failure().stderr(predicate::str::contains(
        "New(Rc<dyn Any>), \
needs information on how to resolve at execution time",
    ));
    // prints 'org.cmjava2023.Main@a92b32a' (i.e. @<some memory address)
    // since memory address is unpredictable,
    // use regex to at least make sure it looks like a memory address
    // let output =
    //     predicate::str::is_match("org\\.cmjava2023\\.Main@[a-zA-Z0-9]*\n")
    //         .unwrap();
    // cmd.assert().success().stdout(output);

    Ok(())
}
