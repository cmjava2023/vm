use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn instance_function() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cmjava")?;

    cmd.arg("tests/data/instance_function/Main.class");
    // contains both an assert.failure() and an assert.success(),
    // so that failure() can be simply removed when all features
    // are implemented
    cmd.assert().failure().stderr(predicate::str::contains(
        "PutField(Rc<dyn Any>), \
needs information on how to resolve fields at execution time",
    ));
    // cmd.assert()
    //     .success()
    //     .stdout(predicate::str::contains("10\n"));

    Ok(())
}
