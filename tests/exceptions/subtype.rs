use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn try_catch() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cmjava")?;

    cmd.arg("tests/data/exceptions/subtype/A.class");
    cmd.arg("tests/data/exceptions/subtype/Main.class");
    cmd.assert().success().stdout(predicate::str::contains(
        "caught e:\nOops\ndid not catch e:\nHuh?",
    ));

    Ok(())
}
