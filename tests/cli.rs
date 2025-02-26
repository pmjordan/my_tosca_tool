use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs


#[test]
fn wrong_args () -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("my_tosca_tool")?;

    cmd.arg("-x").arg("notthere");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage: my_tosca_tool [OPTIONS]"));

    Ok(())
}

#[test]
fn version () -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("my_tosca_tool")?;

    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("my_tosca_tool"));

    Ok(())
}

#[test]
fn file_missing() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("my_tosca_tool")?;

    cmd.arg("-t").arg("examples/not_there.yaml");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("examples/not_there"));
    Ok(())
}

#[test]
fn file_ok() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("my_tosca_tool")?;

    cmd.arg("-t").arg("examples/valid.yaml");
    cmd.assert()
        .success();
    Ok(())
}

#[test]
fn file_not_ok() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("my_tosca_tool")?;

    cmd.arg("-t").arg("examples/invalid.yaml");
    cmd.assert()
        .failure();
    Ok(())
}

#[test]
fn file_not_deployable() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("my_tosca_tool")?;

    cmd.arg("-t").arg("examples/undeployable.yaml");
    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("TOSCA file is valid but not deployable"));
    Ok(())
}