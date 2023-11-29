use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn conversions() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cmjava")?;

    cmd.arg("tests/data/primitive/int/conversions/Main.class");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Unsupported Opcode"));

    Ok(())
}

#[test]
fn mathops() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cmjava")?;

    cmd.arg("tests/data/primitive/int/mathops/Main.class");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Unsupported Opcode"));

    Ok(())
}

#[test]
fn logicops() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cmjava")?;

    cmd.arg("tests/data/primitive/int/logicops/Main.class");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Unsupported Opcode"));

    Ok(())
}
