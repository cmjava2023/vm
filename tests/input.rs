use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn input() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cmjava")?;

    cmd.arg("tests/data/input/Main.class");
    cmd.write_stdin("A");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("65\n"));

    Ok(())
}
