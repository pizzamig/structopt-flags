extern crate assert_cmd;

use assert_cmd::prelude::*;
use std::process::Command;

#[test]
fn test_hostipv4_1() {
    let mut cmd = Command::cargo_example("hostipv4").unwrap();
    let output = cmd.unwrap();
    output.clone().assert().success();
    output.assert().stdout("127.0.0.1\n");
}

#[test]
fn test_hostipv4_2() {
    let mut cmd = Command::cargo_example("hostipv4").unwrap();
    let output = cmd.args(&["-H", "10.0.0.1"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("10.0.0.1\n");
}

#[test]
fn test_hostipv4_3() {
    let mut cmd = Command::cargo_example("hostipv4").unwrap();
    cmd.args(&["-H", "::1"]).assert().failure();
}

#[test]
fn test_hostipv4_4() {
    let mut cmd = Command::cargo_example("hostipv4").unwrap();
    cmd.args(&["-H", "none"]).assert().failure();
}

#[test]
fn test_hostipv4_5() {
    let mut cmd = Command::cargo_example("hostipv4").unwrap();
    cmd.args(&["-H", "256.0.0.1"]).assert().failure();
}

#[test]
fn test_hostipv4param_1() {
    let mut cmd = Command::cargo_example("hostipv4_param").unwrap();
    cmd.assert().failure();
}

#[test]
fn test_hostipv4param_2() {
    let mut cmd = Command::cargo_example("hostipv4_param").unwrap();
    let output = cmd.args(&["-H", "10.0.0.1"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("10.0.0.1\n");
}

#[test]
fn test_hostipv4param_3() {
    let mut cmd = Command::cargo_example("hostipv4_param").unwrap();
    cmd.args(&["-H", "::1"]).assert().failure();
}

#[test]
fn test_hostipv4param_4() {
    let mut cmd = Command::cargo_example("hostipv4_param").unwrap();
    cmd.args(&["-H", "none"]).assert().failure();
}

#[test]
fn test_hostipv4param_5() {
    let mut cmd = Command::cargo_example("hostipv4_param").unwrap();
    cmd.args(&["-H", "256.0.0.1"]).assert().failure();
}
#[test]
fn test_hostipv6_1() {
    let mut cmd = Command::cargo_example("hostipv6").unwrap();
    let output = cmd.unwrap();
    output.clone().assert().success();
    output.assert().stdout("::1\n");
}

#[test]
fn test_hostipv6_2() {
    let mut cmd = Command::cargo_example("hostipv6").unwrap();
    let output = cmd.args(&["-H", "200a::1"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("200a::1\n");
}

#[test]
fn test_hostipv6_3() {
    let mut cmd = Command::cargo_example("hostipv6").unwrap();
    cmd.args(&["-H", "127.0.0.1"]).assert().failure();
}

#[test]
fn test_hostipv6_4() {
    let mut cmd = Command::cargo_example("hostipv6").unwrap();
    cmd.args(&["-H", "none"]).assert().failure();
}

#[test]
fn test_hostipv6_5() {
    let mut cmd = Command::cargo_example("hostipv6").unwrap();
    cmd.args(&["-H", "200a::1::1"]).assert().failure();
}

#[test]
fn test_hostipv6param_1() {
    let mut cmd = Command::cargo_example("hostipv6_param").unwrap();
    cmd.assert().failure();
}

#[test]
fn test_hostipv6param_2() {
    let mut cmd = Command::cargo_example("hostipv6_param").unwrap();
    let output = cmd.args(&["-H", "200a::1"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("200a::1\n");
}

#[test]
fn test_hostipv6param_3() {
    let mut cmd = Command::cargo_example("hostipv6_param").unwrap();
    cmd.args(&["-H", "127.0.0.1"]).assert().failure();
}

#[test]
fn test_hostipv6param_4() {
    let mut cmd = Command::cargo_example("hostipv6_param").unwrap();
    cmd.args(&["-H", "none"]).assert().failure();
}

#[test]
fn test_hostipv6param_5() {
    let mut cmd = Command::cargo_example("hostipv6_param").unwrap();
    cmd.args(&["-H", "200a::1::1"]).assert().failure();
}

#[test]
fn test_hostip_1() {
    let mut cmd = Command::cargo_example("hostip").unwrap();
    let output = cmd.unwrap();
    output.clone().assert().success();
    output.assert().stdout("::1\n");
}

#[test]
fn test_hostip_2() {
    let mut cmd = Command::cargo_example("hostip").unwrap();
    let output = cmd.args(&["-H", "200a::1"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("200a::1\n");
}

#[test]
fn test_hostip_3() {
    let mut cmd = Command::cargo_example("hostip").unwrap();
    let output = cmd.args(&["-H", "127.0.0.1"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("127.0.0.1\n");
}

#[test]
fn test_hostip_4() {
    let mut cmd = Command::cargo_example("hostip").unwrap();
    cmd.args(&["-H", "256.0.0.1"]).assert().failure();
}

#[test]
fn test_hostip_5() {
    let mut cmd = Command::cargo_example("hostip").unwrap();
    cmd.args(&["-H", "none"]).assert().failure();
}

#[test]
fn test_hostip_6() {
    let mut cmd = Command::cargo_example("hostip").unwrap();
    cmd.args(&["-H", "200a::1::1"]).assert().failure();
}

#[test]
fn test_hostipparam_1() {
    let mut cmd = Command::cargo_example("hostip_param").unwrap();
    cmd.assert().failure();
}

#[test]
fn test_hostipparam_2() {
    let mut cmd = Command::cargo_example("hostip_param").unwrap();
    let output = cmd.args(&["-H", "200a::1"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("200a::1\n");
}

#[test]
fn test_hostipparam_3() {
    let mut cmd = Command::cargo_example("hostip_param").unwrap();
    let output = cmd.args(&["-H", "127.0.0.1"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("127.0.0.1\n");
}

#[test]
fn test_hostipparam_4() {
    let mut cmd = Command::cargo_example("hostip_param").unwrap();
    cmd.args(&["-H", "256.0.0.1"]).assert().failure();
}

#[test]
fn test_hostipparam_5() {
    let mut cmd = Command::cargo_example("hostip_param").unwrap();
    cmd.args(&["-H", "none"]).assert().failure();
}

#[test]
fn test_hostipparam_6() {
    let mut cmd = Command::cargo_example("hostip_param").unwrap();
    cmd.args(&["-H", "200a::1::1"]).assert().failure();
}
