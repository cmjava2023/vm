use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn conversions() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cmjava")?;

    cmd.arg("tests/data/primitive/float/conversions/Main.class");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("f:\n10"))
        .stdout(predicate::str::contains("d:\n10"))
        .stdout(predicate::str::contains("l:\n10"))
        .stdout(predicate::str::contains("i:\n10"))
        .stdout(predicate::str::contains("force_load:\n10"))
        .stdout(predicate::str::contains("force_store:\n10"))
        .stdout(predicate::str::contains("force_const:\n1"));

    Ok(())
}

#[test]
fn mathops() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cmjava")?;

    cmd.arg("tests/data/primitive/float/mathops/Main.class");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("f:\n10"))
        .stdout(predicate::str::contains("f2:\n20"))
        .stdout(predicate::str::contains("f3:\n10"))
        .stdout(predicate::str::contains("f4:\n40"))
        .stdout(predicate::str::contains("f5:\n-20"))
        .stdout(predicate::str::contains("f6:\n0"))
        .stdout(predicate::str::contains("f7:\n0"));

    Ok(())
}
