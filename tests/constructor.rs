use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn constructor() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cmjava")?;

    cmd.arg("tests/data/constructor/Main.class");
    // prints 'org.cmjava2023.Main@a92b32a' (i.e. @<some memory address)
    // since memory address is unpredictable,
    // use regex to at least make sure it looks like a memory address
    let output =
        predicate::str::is_match("org/cmjava2023/Main@0x[a-zA-Z0-9]*\n")
            .unwrap();
    cmd.assert().success().stdout(output);

    Ok(())
}
