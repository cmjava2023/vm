use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn hello_world() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cmjava")?;

    cmd.arg("data/hello_world/HelloWorld.class");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Hello world!\n"));

    Ok(())
}
