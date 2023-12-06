use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn conversions() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cmjava")?;

    cmd.arg("tests/data/primitive/long/conversions/Main.class");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Missing OpCode implementation"));

    Ok(())
}

#[test]
fn mathops() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cmjava")?;

    cmd.arg("tests/data/primitive/long/mathops/Main.class");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Missing OpCode implementation"));

    Ok(())
}

#[test]
fn logicops() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cmjava")?;

    cmd.arg("tests/data/primitive/long/logicops/Main.class");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Missing OpCode implementation"));

    Ok(())
}
