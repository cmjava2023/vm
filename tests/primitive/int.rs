use assert_cmd::Command;

#[test]
fn conversions() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cmjava")?;

    cmd.arg("tests/data/primitive/int/conversions/Main.class");
    cmd.assert().success();

    Ok(())
}

#[test]
fn mathops() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cmjava")?;

    cmd.arg("tests/data/primitive/int/mathops/Main.class");
    cmd.assert().success();

    Ok(())
}

#[test]
fn logicops() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cmjava")?;

    cmd.arg("tests/data/primitive/int/logicops/Main.class");
    cmd.assert().success();

    Ok(())
}
