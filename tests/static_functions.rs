use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn static_functions() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("cmjava")?;

    cmd.arg("tests/data/static_functions/Main.class");
    // prints '[I@a92b32a' (i.e. @<some memory address)
    // since memory address is unpredictable,
    // use regex to at least make sure it looks like a memory address
    let nums_output = predicate::str::is_match(
        "\\(main\\) nums:\n\\[I@0x[a-zA-Z0-9]*
\\(arrayArg\\) nums:\n\\[I@0x[a-zA-Z0-9]*\n",
    )
    .unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::contains(
            "(main) greeting:\nHello World
(objectArg) greeting:\nHello World\n",
        ))
        .stdout(predicate::str::contains(
            "(main) num:\n10\n(primitiveArg) num:\n10\n",
        ))
        .stdout(nums_output)
        .stdout(predicate::str::contains(
            "(main) d:\n10\n(largePrimitiveArg) d:\n10\n",
        ));

    Ok(())
}
