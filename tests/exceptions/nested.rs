use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn try_catch_finally_throwable() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cmjava")?;

    cmd.arg("tests/data/exceptions/nested/Main.class");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("caught e:\nOops\nanyway\n"));

    Ok(())
}
