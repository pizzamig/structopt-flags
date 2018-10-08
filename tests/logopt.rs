extern crate assert_cmd;

use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn test_logopt_1() {
    let mut cmd = Command::cargo_example("logopt").unwrap();
    let output = cmd.unwrap();
    output.clone().assert().success();
    output.assert().stdout("INFO\n");
}

#[test]
fn test_logopt_2() {
    let mut cmd = Command::cargo_example("logopt").unwrap();
    let output = cmd.args(&["-L", "WARN"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("WARN\n");
}

#[test]
fn test_logopt_3() {
    let mut cmd = Command::cargo_example("logopt").unwrap();
    cmd.args(&["-L", "none"]).assert().failure();
}

#[test]
fn test_logopt_4() {
    let mut cmd = Command::cargo_example("logopt").unwrap();
    cmd.args(&["-l"]).assert().failure();
}

#[test]
fn test_logoptlower_1() {
    let mut cmd = Command::cargo_example("logoptlower").unwrap();
    let output = cmd.unwrap();
    output.clone().assert().success();
    output.assert().stdout("INFO\n");
}

#[test]
fn test_logoptlower_2() {
    let mut cmd = Command::cargo_example("logoptlower").unwrap();
    let output = cmd.args(&["-l", "WARN"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("WARN\n");
}

#[test]
fn test_logoptlower_3() {
    let mut cmd = Command::cargo_example("logoptlower").unwrap();
    cmd.args(&["-l", "none"]).assert().failure();
}

#[test]
fn test_logoptlower_4() {
    let mut cmd = Command::cargo_example("logoptlower").unwrap();
    cmd.args(&["-L"]).assert().failure();
}
