use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn simple_polymorphism() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cmjava")?;

    cmd.arg("tests/data/inheritance/simple_with_polymorphism/A.class");
    cmd.arg("tests/data/inheritance/simple_with_polymorphism/B.class");
    cmd.arg("tests/data/inheritance/simple_with_polymorphism/Main.class");
    cmd.assert().success().stdout(predicate::str::contains(
        "(A) doStuff()\n(B) doStuff()\n(A) doOtherStuff()\n",
    ));

    Ok(())
}
