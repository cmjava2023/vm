use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn conversions() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cmjava")?;

    cmd.arg("tests/data/primitive/long/conversions/Main.class");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("l:\n10\n"))
        .stdout(predicate::str::contains("d:\n10\n"))
        .stdout(predicate::str::contains("f:\n10\n"))
        .stdout(predicate::str::contains("i:\n10\n"))
        .stdout(predicate::str::contains("force_load:\n10\n"))
        .stdout(predicate::str::contains("force_store:\n10\n"))
        .stdout(predicate::str::contains("force_const:\n1\n"));

    Ok(())
}

#[test]
fn mathops() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cmjava")?;

    cmd.arg("tests/data/primitive/long/mathops/Main.class");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("l:\n10\n"))
        .stdout(predicate::str::contains("l2:\n20\n"))
        .stdout(predicate::str::contains("l3:\n10\n"))
        .stdout(predicate::str::contains("l4:\n40\n"))
        .stdout(predicate::str::contains("l5:\n-20\n"))
        .stdout(predicate::str::contains("l6:\n0\n"))
        .stdout(predicate::str::contains("l7:\n0\n"));

    Ok(())
}

#[test]
fn logicops() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cmjava")?;

    cmd.arg("tests/data/primitive/long/logicops/Main.class");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("l:\n4\n"))
        .stdout(predicate::str::contains("eight:\n8\n"))
        .stdout(predicate::str::contains("l2:\n0\n"))
        .stdout(predicate::str::contains("l3:\n6\n"))
        .stdout(predicate::str::contains("l4:\n16\n"))
        .stdout(predicate::str::contains("l5:\n4\n"))
        .stdout(predicate::str::contains("l6:\n4\n"))
        .stdout(predicate::str::contains("l7:\n2\n"));

    Ok(())
}
