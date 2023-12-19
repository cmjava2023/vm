use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn input() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cmjava")?;
    let sample_input = 'A';

    cmd.arg("tests/data/input/Main.class");
    cmd.write_stdin(format!("{}", sample_input));
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(format!(
            "{}\n",
            sample_input as u8
        )));

    Ok(())
}
