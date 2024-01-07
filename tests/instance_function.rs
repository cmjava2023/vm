use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn instance_function() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cmjava")?;

    cmd.arg("tests/data/instance_function/Main.class");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("10\n"));

    Ok(())
}
