use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn simple_inheritance() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cmjava")?;

    cmd.arg("tests/data/inheritance/simple/A.class");
    cmd.arg("tests/data/inheritance/simple/B.class");
    cmd.arg("tests/data/inheritance/simple/Main.class");
    // contains both an assert.failure() and an assert.success(),
    // so that failure() can be simply removed when all features
    // are implemented
    cmd.assert().failure().stderr(predicate::str::contains(
        "New(Rc<dyn Any>), \
needs information on how to resolve at execution time",
    ));
    // cmd.assert().success().stdout(predicate::str::contains(
    //     "(A) doStuff()\n(B) doStuff()\n(A) doOtherStuff()\n",
    // ));

    Ok(())
}
