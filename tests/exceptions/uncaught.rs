use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn try_catch_finally_throwable() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cmjava")?;

    let stderr = predicate::str::is_match(
        "Uncaught exception: instance of Class 'java/lang/Throwable'",
    )
    .unwrap();

    cmd.arg("tests/data/exceptions/uncaught/Main.class");
    cmd.assert()
        .failure()
        .stderr(stderr)
        .stdout(predicate::str::contains("anyway"));

    Ok(())
}
