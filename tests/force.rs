use assert_cmd::prelude::*;
use escargot::CargoBuild;

#[test]
fn test_force_1() {
    let example = CargoBuild::new().example("forceflag").run().unwrap();
    let mut cmd = example.command();
    let output = cmd.unwrap();
    output.clone().assert().success();
    output.assert().stdout("force: False\n");
}

#[test]
fn test_force_2() {
    let example = CargoBuild::new().example("forceflag").run().unwrap();
    let mut cmd = example.command();
    let output = cmd.args(&["-f"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("force: True\n");
}

#[test]
fn test_force_3() {
    let example = CargoBuild::new().example("forceflag").run().unwrap();
    let mut cmd = example.command();
    let output = cmd.args(&["--force"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("force: True\n");
}

#[test]
fn test_force_4() {
    let example = CargoBuild::new().example("forceflag").run().unwrap();
    let mut cmd = example.command();
    cmd.args(&["-F"]).assert().failure();
}
