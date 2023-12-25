use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn try_catch_finally_throwable() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cmjava")?;

    cmd.arg("tests/data/exceptions/simple/Main.class");
    // contains both an assert.failure() and an assert.success(),
    // so that failure() can be simply removed when all features
    // are implemented
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Unknown Attribute StackMapTable"));
    // cmd.assert()
    //     .success()
    //     .stdout(predicate::str::contains("caught e:\nOops\nanyway\n"));

    Ok(())
}
