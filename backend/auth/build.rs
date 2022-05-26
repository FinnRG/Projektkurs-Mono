use std::env;
use std::error::Error;
use std::process::{exit, Command};

fn main() -> Result<(), Box<dyn Error>> {
    let buf_enabled = env::var("RUN_BUF").map(|v| v == "1").unwrap_or(true);
    if buf_enabled {
        return run_buf();
    }
    Ok(())
}

fn run_buf() -> Result<(), Box<dyn Error>> {
    let status = Command::new("buf")
        .arg("generate")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .status()
        .unwrap();

    if !status.success() {
        exit(status.code().unwrap_or(-1))
    }

    Ok(())
}
