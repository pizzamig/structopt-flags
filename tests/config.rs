use assert_cmd::prelude::*;
use escargot::CargoBuild;

#[test]
fn test_config_1() {
    let example = CargoBuild::new().example("config").run().unwrap();
    let mut cmd = example.command();
    cmd.assert().failure();
}

#[test]
fn test_config_2() {
    let example = CargoBuild::new().example("config").run().unwrap();
    let mut cmd = example.command();
    let output = cmd.args(&["-c", "bluto.txt"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("bluto.txt\n");
}

#[test]
fn test_config_3() {
    let example = CargoBuild::new().example("config").run().unwrap();
    let mut cmd = example.command();
    let output = cmd.args(&["--config", "bluto.txt"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("bluto.txt\n");
}

#[test]
fn test_confignodef_1() {
    let example = CargoBuild::new()
        .example("config_no_default")
        .run()
        .unwrap();
    let mut cmd = example.command();
    let output = cmd.unwrap();
    output.clone().assert().success();
    output.assert().stdout("None\n");
}

#[test]
fn test_confignodef_2() {
    let example = CargoBuild::new()
        .example("config_no_default")
        .run()
        .unwrap();
    let mut cmd = example.command();
    let output = cmd.args(&["-c", "bluto.txt"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("bluto.txt\n");
}

#[test]
fn test_confignodef_3() {
    let example = CargoBuild::new()
        .example("config_no_default")
        .run()
        .unwrap();
    let mut cmd = example.command();
    let output = cmd.args(&["--config", "bluto.txt"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("bluto.txt\n");
}
