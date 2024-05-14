use anyhow::Result;
use std::process::Command;

#[test]
fn main() -> Result<()> {
    let status = Command::new("cargo")
        .current_dir("tests/program")
        .arg("component")
        .arg("build")
        .arg("--quiet")
        .status()?;
    assert!(status.success());

    let status = Command::new("wasmtime")
        .arg("-S")
        .arg("http")
        .arg("tests/program/target/wasm32-wasi/debug/wasi_http_client_test_program.wasm")
        .status()?;
    assert!(status.success());

    Ok(())
}
