use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn array() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cmjava")?;

    cmd.arg("tests/data/array/Main.class");
    cmd.assert().failure().stderr(predicate::str::contains(
        "needs information on how to resolve",
    ));

    Ok(())
}
