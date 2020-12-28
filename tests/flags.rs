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

#[test]
fn test_dry_run_1() {
    let example = CargoBuild::new().example("dryrunflag").run().unwrap();
    let mut cmd = example.command();
    let output = cmd.unwrap();
    output.clone().assert().success();
    output.assert().stdout("dry_run: False\n");
}

#[test]
fn test_dry_run_2() {
    let example = CargoBuild::new().example("dryrunflag").run().unwrap();
    let mut cmd = example.command();
    let output = cmd.args(&["-n"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("dry_run: True\n");
}

#[test]
fn test_dry_run_3() {
    let example = CargoBuild::new().example("dryrunflag").run().unwrap();
    let mut cmd = example.command();
    let output = cmd.args(&["--dry-run"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("dry_run: True\n");
}

#[test]
fn test_dry_run_4() {
    let example = CargoBuild::new().example("dryrunflag").run().unwrap();
    let mut cmd = example.command();
    cmd.args(&["--dryrun"]).assert().failure();
}

#[test]
fn test_yes_1() {
    let example = CargoBuild::new().example("yesflag").run().unwrap();
    let mut cmd = example.command();
    let output = cmd.unwrap();
    output.clone().assert().success();
    output.assert().stdout("yes: False\n");
}

#[test]
fn test_yes_2() {
    let example = CargoBuild::new().example("yesflag").run().unwrap();
    let mut cmd = example.command();
    let output = cmd.args(&["-y"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("yes: True\n");
}

#[test]
fn test_yes_3() {
    let example = CargoBuild::new().example("yesflag").run().unwrap();
    let mut cmd = example.command();
    let output = cmd.args(&["--yes"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("yes: True\n");
}

#[test]
fn test_yes_4() {
    let example = CargoBuild::new().example("yesflag").run().unwrap();
    let mut cmd = example.command();
    cmd.args(&["-Y"]).assert().failure();
}
