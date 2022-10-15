use assert_cmd::prelude::*;
use colored::ColoredString;
use predicates::prelude::*;
use rmt_lib::argument_errors::{RmtArgumentErrors};
use std::{process::Command, os::unix::process::CommandExt};
use colored::*;

#[test]
fn test_no_args() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    // let expected_output = RmtArgumentErrors::InvalidNumberOfArguments(0).to_string();
    println!("{}", ColoredString::
    // cmd.assert()
    //     .failure()
    //     .stdout(predicate::str::diff(expected_output));
}
