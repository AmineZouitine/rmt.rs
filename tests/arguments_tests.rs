use assert_cmd::prelude::*;
use predicates::prelude::*;
use rmt_lib::argument_errors::{RmtArgumentErrors};
use std::{process::Command, os::unix::process::CommandExt};

#[test]
fn test_no_args() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let expected_output = RmtArgumentErrors::InvalidNumberOfArguments(0).to_string();
    // cmd.assert()
    //     .failure()
    //     .stdout(predicate::str::diff(expected_output));
}
