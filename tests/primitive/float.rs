use assert_cmd::Command;

#[test]
fn conversions() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cmjava")?;

    cmd.arg("tests/data/primitive/float/conversions/Main.class");
    cmd.assert().failure();

    Ok(())
}

#[test]
fn mathops() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cmjava")?;

    cmd.arg("tests/data/primitive/float/mathops/Main.class");
    cmd.assert().failure();

    Ok(())
}

#[test]
fn logicops() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cmjava")?;

    cmd.arg("tests/data/primitive/float/logicops/Main.class");
    cmd.assert().failure();

    Ok(())
}
