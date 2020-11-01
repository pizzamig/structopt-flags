use assert_cmd::prelude::*;
use escargot::CargoBuild;

#[test]
fn test_hostipv4_1() {
    let example = CargoBuild::new().example("hostipv4").run().unwrap();
    let mut cmd = example.command();
    let output = cmd.unwrap();
    output.clone().assert().success();
    output.assert().stdout("127.0.0.1\n");
}

#[test]
fn test_hostipv4_2() {
    let example = CargoBuild::new().example("hostipv4").run().unwrap();
    let mut cmd = example.command();
    let output = cmd.args(&["-H", "10.0.0.1"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("10.0.0.1\n");
}

#[test]
fn test_hostipv4_3() {
    let example = CargoBuild::new().example("hostipv4").run().unwrap();
    let mut cmd = example.command();
    cmd.args(&["-H", "::1"]).assert().failure();
}

#[test]
fn test_hostipv4_4() {
    let example = CargoBuild::new().example("hostipv4").run().unwrap();
    let mut cmd = example.command();
    cmd.args(&["-H", "none"]).assert().failure();
}

#[test]
fn test_hostipv4_5() {
    let example = CargoBuild::new().example("hostipv4").run().unwrap();
    let mut cmd = example.command();
    cmd.args(&["-H", "256.0.0.1"]).assert().failure();
}

#[test]
fn test_hostipv4param_1() {
    let example = CargoBuild::new().example("hostipv4_param").run().unwrap();
    let mut cmd = example.command();
    cmd.assert().failure();
}

#[test]
fn test_hostipv4param_2() {
    let example = CargoBuild::new().example("hostipv4_param").run().unwrap();
    let mut cmd = example.command();
    let output = cmd.args(&["-H", "10.0.0.1"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("10.0.0.1\n");
}

#[test]
fn test_hostipv4param_3() {
    let example = CargoBuild::new().example("hostipv4_param").run().unwrap();
    let mut cmd = example.command();
    cmd.args(&["-H", "::1"]).assert().failure();
}

#[test]
fn test_hostipv4param_4() {
    let example = CargoBuild::new().example("hostipv4_param").run().unwrap();
    let mut cmd = example.command();
    cmd.args(&["-H", "none"]).assert().failure();
}

#[test]
fn test_hostipv4param_5() {
    let example = CargoBuild::new().example("hostipv4_param").run().unwrap();
    let mut cmd = example.command();
    cmd.args(&["-H", "256.0.0.1"]).assert().failure();
}
#[test]
fn test_hostipv6_1() {
    let example = CargoBuild::new().example("hostipv6").run().unwrap();
    let mut cmd = example.command();
    let output = cmd.unwrap();
    output.clone().assert().success();
    output.assert().stdout("::1\n");
}

#[test]
fn test_hostipv6_2() {
    let example = CargoBuild::new().example("hostipv6").run().unwrap();
    let mut cmd = example.command();
    let output = cmd.args(&["-H", "200a::1"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("200a::1\n");
}

#[test]
fn test_hostipv6_3() {
    let example = CargoBuild::new().example("hostipv6").run().unwrap();
    let mut cmd = example.command();
    cmd.args(&["-H", "127.0.0.1"]).assert().failure();
}

#[test]
fn test_hostipv6_4() {
    let example = CargoBuild::new().example("hostipv6").run().unwrap();
    let mut cmd = example.command();
    cmd.args(&["-H", "none"]).assert().failure();
}

#[test]
fn test_hostipv6_5() {
    let example = CargoBuild::new().example("hostipv6").run().unwrap();
    let mut cmd = example.command();
    cmd.args(&["-H", "200a::1::1"]).assert().failure();
}

#[test]
fn test_hostipv6param_1() {
    let example = CargoBuild::new().example("hostipv6_param").run().unwrap();
    let mut cmd = example.command();
    cmd.assert().failure();
}

#[test]
fn test_hostipv6param_2() {
    let example = CargoBuild::new().example("hostipv6_param").run().unwrap();
    let mut cmd = example.command();
    let output = cmd.args(&["-H", "200a::1"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("200a::1\n");
}

#[test]
fn test_hostipv6param_3() {
    let example = CargoBuild::new().example("hostipv6_param").run().unwrap();
    let mut cmd = example.command();
    cmd.args(&["-H", "127.0.0.1"]).assert().failure();
}

#[test]
fn test_hostipv6param_4() {
    let example = CargoBuild::new().example("hostipv6_param").run().unwrap();
    let mut cmd = example.command();
    cmd.args(&["-H", "none"]).assert().failure();
}

#[test]
fn test_hostipv6param_5() {
    let example = CargoBuild::new().example("hostipv6_param").run().unwrap();
    let mut cmd = example.command();
    cmd.args(&["-H", "200a::1::1"]).assert().failure();
}

#[test]
fn test_hostip_1() {
    let example = CargoBuild::new().example("hostip").run().unwrap();
    let mut cmd = example.command();
    let output = cmd.unwrap();
    output.clone().assert().success();
    output.assert().stdout("::1\n");
}

#[test]
fn test_hostip_2() {
    let example = CargoBuild::new().example("hostip").run().unwrap();
    let mut cmd = example.command();
    let output = cmd.args(&["-H", "200a::1"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("200a::1\n");
}

#[test]
fn test_hostip_3() {
    let example = CargoBuild::new().example("hostip").run().unwrap();
    let mut cmd = example.command();
    let output = cmd.args(&["-H", "127.0.0.1"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("127.0.0.1\n");
}

#[test]
fn test_hostip_4() {
    let example = CargoBuild::new().example("hostip").run().unwrap();
    let mut cmd = example.command();
    cmd.args(&["-H", "256.0.0.1"]).assert().failure();
}

#[test]
fn test_hostip_5() {
    let example = CargoBuild::new().example("hostip").run().unwrap();
    let mut cmd = example.command();
    cmd.args(&["-H", "none"]).assert().failure();
}

#[test]
fn test_hostip_6() {
    let example = CargoBuild::new().example("hostip").run().unwrap();
    let mut cmd = example.command();
    cmd.args(&["-H", "200a::1::1"]).assert().failure();
}

#[test]
fn test_hostipparam_1() {
    let example = CargoBuild::new().example("hostip_param").run().unwrap();
    let mut cmd = example.command();
    cmd.assert().failure();
}

#[test]
fn test_hostipparam_2() {
    let example = CargoBuild::new().example("hostip_param").run().unwrap();
    let mut cmd = example.command();
    let output = cmd.args(&["-H", "200a::1"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("200a::1\n");
}

#[test]
fn test_hostipparam_3() {
    let example = CargoBuild::new().example("hostip_param").run().unwrap();
    let mut cmd = example.command();
    let output = cmd.args(&["-H", "127.0.0.1"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("127.0.0.1\n");
}

#[test]
fn test_hostipparam_4() {
    let example = CargoBuild::new().example("hostip_param").run().unwrap();
    let mut cmd = example.command();
    cmd.args(&["-H", "256.0.0.1"]).assert().failure();
}

#[test]
fn test_hostipparam_5() {
    let example = CargoBuild::new().example("hostip_param").run().unwrap();
    let mut cmd = example.command();
    cmd.args(&["-H", "none"]).assert().failure();
}

#[test]
fn test_hostipparam_6() {
    let example = CargoBuild::new().example("hostip_param").run().unwrap();
    let mut cmd = example.command();
    cmd.args(&["-H", "200a::1::1"]).assert().failure();
}
