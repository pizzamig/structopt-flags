extern crate assert_cmd;

use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn test_verbose_1() {
    let mut cmd = Command::cargo_example("verbose").unwrap();
    let output = cmd.unwrap();
    output.clone().assert().success();
    output.assert().stdout("ERROR\n");
}

#[test]
fn test_verbose_2() {
    let mut cmd = Command::cargo_example("verbose").unwrap();
    let output = cmd.args(&["-v"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("WARN\n");
}

#[test]
fn test_verbose_3() {
    let mut cmd = Command::cargo_example("verbose").unwrap();
    let output = cmd.args(&["-vvvvvv"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("TRACE\n");
}

#[test]
fn test_verbose_4() {
    let mut cmd = Command::cargo_example("verbose").unwrap();
    cmd.args(&["-q"]).assert().failure();
}

#[test]
fn test_quietverbose_1() {
    let mut cmd = Command::cargo_example("quiet_verbose").unwrap();
    let output = cmd.unwrap();
    output.clone().assert().success();
    output.assert().stdout("WARN\n");
}

#[test]
fn test_quietverbose_2() {
    let mut cmd = Command::cargo_example("quiet_verbose").unwrap();
    let output = cmd.args(&["-vv"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("DEBUG\n");
}

#[test]
fn test_quietverbose_3() {
    let mut cmd = Command::cargo_example("quiet_verbose").unwrap();
    let output = cmd.args(&["-qq"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("OFF\n");
}

#[test]
fn test_quietverbose_4() {
    let mut cmd = Command::cargo_example("quiet_verbose").unwrap();
    cmd.args(&["-q", "-v"]).assert().failure();
}

#[test]
fn test_simpleverbose_1() {
    let mut cmd = Command::cargo_example("simple_verbose").unwrap();
    let output = cmd.unwrap();
    output.clone().assert().success();
    output.assert().stdout("False\n");
}

#[test]
fn test_simpleverbose_2() {
    let mut cmd = Command::cargo_example("simple_verbose").unwrap();
    let output = cmd.args(&["-v"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("True\n");
}

#[test]
fn test_simpleverbose_3() {
    let mut cmd = Command::cargo_example("simple_verbose").unwrap();
    cmd.args(&["-vv"]).assert().failure();
}
