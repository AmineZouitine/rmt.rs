use assert_cmd::prelude::*;
use predicates::prelude::*;
use rmt_lib::argument_errors::RmtArgumentErrors;
use std::process::Command;
#[test]
fn test_no_args() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let expected_output = format!("{}\n", RmtArgumentErrors::InvalidNumberOfArguments(0));

    cmd.assert()
        .failure()
        .stdout(predicate::str::diff(expected_output));
}

#[test]
fn test_only_flags() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    cmd.arg("-f");

    let expected_output = format!("{}\n", RmtArgumentErrors::InvalidNumberOfArguments(0));
    cmd.assert()
        .failure()
        .stdout(predicate::str::diff(expected_output));
}

#[test]
fn test_not_existing_file_without_force() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    let not_existing_file_name = "rmt_not_existing_file.rmt".to_string();
    // I hope you don't have a file name like that
    cmd.arg(&not_existing_file_name);

    let expected_output = format!(
        "{}\n",
        RmtArgumentErrors::InvalidPathWithoutForceFlags {
            element_name: not_existing_file_name
        }
    );
    cmd.assert()
        .failure()
        .stdout(predicate::str::diff(expected_output));
}
