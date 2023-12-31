use std::process::Command;
use assert_cmd::prelude::*;
use predicates::prelude::*;


#[test] 
fn run_with_defaults() -> Result<() , Box<dyn std::error::Error>> {
    Command::cargo_bin("horsesay")
        .expect("binary exists")
        .assert()
        .success()
        .stdout(predicate::str::contains("Moo!"));

    Command::cargo_bin("horsesay")
        .expect("binary exists")
        .args(&["-f", "no/such/file.txt"])
        .assert()
        .failure();

    Command::cargo_bin("horsesay")
        .expect("binary exists")
        .args(&["-d"])
        .assert()
        .success()
        .stdout(predicate::str::contains("x x"));

    Ok(())
}