use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn two_classes() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cmjava")?;

    cmd.arg("tests/data/two_classes/Auxiliary.class");
    cmd.arg("tests/data/two_classes/Main.class");
    // contains both an assert.failure() and an assert.success(),
    // so that failure() can be simply removed when all features
    // are implemented
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Missing OpCode implementation"));
    // cmd.assert().success().stdout(predicate::str::contains(
    //     "Hello World from Auxiliary.java\n",
    // ));

    Ok(())
}
