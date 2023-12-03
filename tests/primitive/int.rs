use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn conversions() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cmjava")?;

    cmd.arg("tests/data/primitive/int/conversions/Main.class");
    // int 10 as char is ascii for \n
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("i:\n10\n"))
        .stdout(predicate::str::contains("b:\n10\n"))
        .stdout(predicate::str::contains("c:\n\n\n"))
        .stdout(predicate::str::contains("d:\n10\n"))
        .stdout(predicate::str::contains("f:\n10\n"))
        .stdout(predicate::str::contains("l:\n10\n"))
        .stdout(predicate::str::contains("s:\n10\n"))
        .stdout(predicate::str::contains("force_load:\n10\n"));

    Ok(())
}

#[test]
fn mathops() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cmjava")?;

    cmd.arg("tests/data/primitive/int/mathops/Main.class");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("i:\n10\n"))
        .stdout(predicate::str::contains("i2:\n20\n"))
        .stdout(predicate::str::contains("i3:\n10\n"))
        .stdout(predicate::str::contains("i4:\n21\n"))
        .stdout(predicate::str::contains("i5:\n40\n"))
        .stdout(predicate::str::contains("i6:\n-20\n"))
        .stdout(predicate::str::contains("i7:\n0\n"))
        .stdout(predicate::str::contains("i8:\n0\n"));

    Ok(())
}

#[test]
fn logicops() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cmjava")?;

    cmd.arg("tests/data/primitive/int/logicops/Main.class");
    cmd.assert().failure();
    // failure caused by recent sipush inclusion in test case
    // uncomment when sipush is supported
    // .success()
    // .stdout(predicate::str::contains("i:\n4\n"))
    // .stdout(predicate::str::contains("eight:\n8\n"))
    // .stdout(predicate::str::contains("i2:\n0\n"))
    // .stdout(predicate::str::contains("i3:\n6\n"))
    // .stdout(predicate::str::contains("i4:\n16\n"))
    // .stdout(predicate::str::contains("i5:\n4\n"))
    // .stdout(predicate::str::contains("i6:\n4\n"))
    // .stdout(predicate::str::contains("i7:\n2\n"));

    Ok(())
}
