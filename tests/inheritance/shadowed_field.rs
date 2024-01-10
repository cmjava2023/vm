use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn shadowed_field() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cmjava")?;

    cmd.arg("tests/data/inheritance/shadowed_field/A.class");
    cmd.arg("tests/data/inheritance/shadowed_field/B.class");
    cmd.arg("tests/data/inheritance/shadowed_field/Main.class");
    cmd.assert().success().stdout(predicate::str::contains(
        "(B) thing
10
(A) thing
20",
    ));

    Ok(())
}
