use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn array() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cmjava")?;

    cmd.arg("tests/data/array/Main.class");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("o:\nHello World"))
        .stdout(predicate::str::contains("by:\n10"))
        .stdout(predicate::str::contains("boo:\ntrue"))
        .stdout(predicate::str::contains("c:\n\n\n"))
        .stdout(predicate::str::contains("d:\n10"))
        .stdout(predicate::str::contains("f:\n10"))
        .stdout(predicate::str::contains("i:\n10"))
        .stdout(predicate::str::contains("l:\n10"))
        .stdout(predicate::str::contains("i2:\n0"))
        .stdout(predicate::str::contains("s:\n10"))
        .stdout(predicate::str::contains("len:\n10"));

    Ok(())
}
