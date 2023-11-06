use std::error::Error;

use vergen::EmitBuilder;

fn main() -> Result<(), Box<dyn Error>> {
    EmitBuilder::builder()
        .all_build()
        .all_cargo()
        .all_git()
        .git_describe(true, true, None)
        .all_rustc()
        .all_sysinfo()
        .emit()?;
    Ok(())
}
