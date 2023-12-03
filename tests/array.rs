use assert_cmd::Command;

#[test]
fn array() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cmjava")?;

    cmd.arg("tests/data/array/Main.class");
    cmd.assert().failure();

    Ok(())
}
