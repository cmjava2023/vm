use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn control_flow() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cmjava")?;

    cmd.arg("tests/data/control_flow/Main.class");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("i % 10 == 0\n".repeat(10)))
        .stdout(predicate::str::contains("a / 2 == 5\n"))
        .stdout(predicate::str::contains("l / 2 != 5\n"))
        .stdout(predicate::str::contains("d:\n15\n"))
        .stdout(predicate::str::contains("f > 10"));

    Ok(())
}
