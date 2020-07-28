extern crate assert_cmd;
extern crate escargot;
extern crate predicates;

use assert_cmd::prelude::*;
use escargot::CargoBuild;

#[test]
fn test_verbose_1() {
    let example = CargoBuild::new().example("verbose").run().unwrap();
    let mut cmd = example.command();
    let output = cmd.unwrap();
    output.clone().assert().success();
    output.assert().stdout("ERROR\n");
}

#[test]
fn test_verbose_2() {
    let example = CargoBuild::new().example("verbose").run().unwrap();
    let mut cmd = example.command();
    let output = cmd.args(&["-v"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("WARN\n");
}

#[test]
fn test_verbose_3() {
    let example = CargoBuild::new().example("verbose").run().unwrap();
    let mut cmd = example.command();
    let output = cmd.args(&["-vvvvvv"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("TRACE\n");
}

#[test]
fn test_verbose_4() {
    let example = CargoBuild::new().example("verbose").run().unwrap();
    let mut cmd = example.command();
    cmd.args(&["-q"]).assert().failure();
}

#[test]
fn test_verbose_5() {
    let example = CargoBuild::new().example("verbose").run().unwrap();
    let mut cmd = example.command();
    let output = cmd.args(&["-h"]).unwrap();
    output.clone().assert().success();
    output
        .assert()
        .stdout(predicates::str::contains("An example using verbose flag\n"));
}

#[cfg(feature = "simplelog")]
#[test]
fn test_verbose_simplelog_1() {
    let example = CargoBuild::new()
        .example("verbose_simplelog")
        .features("simplelog")
        .run()
        .unwrap();
    let mut cmd = example.command();
    let output = cmd.unwrap();
    output.clone().assert().success();
    output.assert().stdout("");
}

#[cfg(feature = "simplelog")]
#[test]
fn test_verbose_simplelog_2() {
    let example = CargoBuild::new()
        .example("verbose_simplelog")
        .features("simplelog")
        .run()
        .unwrap();
    let mut cmd = example.command();
    let output = cmd.args(&["-v"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("");
}

#[cfg(feature = "simplelog")]
#[test]
fn test_verbose_simplelog_3() {
    let example = CargoBuild::new()
        .example("verbose_simplelog")
        .features("simplelog")
        .run()
        .unwrap();
    let mut cmd = example.command();
    let output = cmd.args(&["-vvvvvv"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout(predicates::str::ends_with(
        "verbose_simplelog: [examples/verbose_simplelog.rs:25] TRACE\n",
    ));
}

#[cfg(feature = "simplelog")]
#[test]
fn test_verbose_simplelog_4() {
    let example = CargoBuild::new()
        .example("verbose_simplelog")
        .features("simplelog")
        .run()
        .unwrap();
    let mut cmd = example.command();
    cmd.args(&["-q"]).assert().failure();
}

#[test]
fn test_verbosenodef_1() {
    let example = CargoBuild::new()
        .example("verbose_no_default")
        .run()
        .unwrap();
    let mut cmd = example.command();
    let output = cmd.unwrap();
    output.clone().assert().success();
    output.assert().stdout("OFF\n");
}

#[test]
fn test_verbosenodef_2() {
    let example = CargoBuild::new()
        .example("verbose_no_default")
        .run()
        .unwrap();
    let mut cmd = example.command();
    let output = cmd.args(&["-v"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("ERROR\n");
}

#[test]
fn test_verbosenodef_3() {
    let example = CargoBuild::new()
        .example("verbose_no_default")
        .run()
        .unwrap();
    let mut cmd = example.command();
    let output = cmd.args(&["-vvvvvvvvv"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("TRACE\n");
}

#[test]
fn test_verbosenodef_4() {
    let example = CargoBuild::new()
        .example("verbose_no_default")
        .run()
        .unwrap();
    let mut cmd = example.command();
    cmd.args(&["-q"]).assert().failure();
}

#[cfg(feature = "simplelog")]
#[test]
fn test_verbosenodef_simplelog_1() {
    let example = CargoBuild::new()
        .example("verbose_no_default_simplelog")
        .features("simplelog")
        .run()
        .unwrap();
    let mut cmd = example.command();
    let output = cmd.unwrap();
    output.clone().assert().success();
    output.assert().stdout("");
}

#[cfg(feature = "simplelog")]
#[test]
fn test_verbosenodef_simplelog_2() {
    let example = CargoBuild::new()
        .example("verbose_no_default_simplelog")
        .features("simplelog")
        .run()
        .unwrap();
    let mut cmd = example.command();
    let output = cmd.args(&["-vvvvvvvvv"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout(predicates::str::ends_with(
        "verbose_no_default_simplelog: [examples/verbose_no_default_simplelog.rs:30] TRACE\n",
    ));
}

#[cfg(feature = "simplelog")]
#[test]
fn test_verbosenodef_simplelog_3() {
    let example = CargoBuild::new()
        .example("verbose_no_default_simplelog")
        .features("simplelog")
        .run()
        .unwrap();
    let mut cmd = example.command();
    cmd.args(&["-q"]).assert().failure();
}

#[test]
fn test_quietverbose_1() {
    let example = CargoBuild::new().example("quiet_verbose").run().unwrap();
    let mut cmd = example.command();
    let output = cmd.unwrap();
    output.clone().assert().success();
    output.assert().stdout("WARN\n");
}

#[test]
fn test_quietverbose_2() {
    let example = CargoBuild::new().example("quiet_verbose").run().unwrap();
    let mut cmd = example.command();
    let output = cmd.args(&["-vv"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("DEBUG\n");
}

#[test]
fn test_quietverbose_3() {
    let example = CargoBuild::new().example("quiet_verbose").run().unwrap();
    let mut cmd = example.command();
    let output = cmd.args(&["-qq"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("OFF\n");
}

#[test]
fn test_quietverbose_4() {
    let example = CargoBuild::new().example("quiet_verbose").run().unwrap();
    let mut cmd = example.command();
    cmd.args(&["-q", "-v"]).assert().failure();
}

#[cfg(feature = "simplelog")]
#[test]
fn test_quietverbose_simplelog_1() {
    let example = CargoBuild::new()
        .example("quiet_verbose_simplelog")
        .features("simplelog")
        .run()
        .unwrap();
    let mut cmd = example.command();
    let output = cmd.unwrap();
    output.clone().assert().success();
    output.assert().stdout("");
}

#[cfg(feature = "simplelog")]
#[test]
fn test_quietverbose_simplelog_2() {
    let example = CargoBuild::new()
        .example("quiet_verbose_simplelog")
        .features("simplelog")
        .run()
        .unwrap();
    let mut cmd = example.command();
    let output = cmd.args(&["-vv"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout(predicates::str::ends_with(
        "quiet_verbose_simplelog: DEBUG\n",
    ));
}

#[cfg(feature = "simplelog")]
#[test]
fn test_quietverbose_simplelog_3() {
    let example = CargoBuild::new()
        .example("quiet_verbose_simplelog")
        .features("simplelog")
        .run()
        .unwrap();
    let mut cmd = example.command();
    let output = cmd.args(&["-qq"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("");
}

#[cfg(feature = "simplelog")]
#[test]
fn test_quietverbose_simplelog_4() {
    let example = CargoBuild::new()
        .example("quiet_verbose_simplelog")
        .features("simplelog")
        .run()
        .unwrap();
    let mut cmd = example.command();
    cmd.args(&["-q", "-v"]).assert().failure();
}

#[test]
fn test_simpleverbose_1() {
    let example = CargoBuild::new().example("simple_verbose").run().unwrap();
    let mut cmd = example.command();
    let output = cmd.unwrap();
    output.clone().assert().success();
    output.assert().stdout("False\n");
}

#[test]
fn test_simpleverbose_2() {
    let example = CargoBuild::new().example("simple_verbose").run().unwrap();
    let mut cmd = example.command();
    let output = cmd.args(&["-v"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("True\n");
}

#[test]
fn test_simpleverbose_3() {
    let example = CargoBuild::new().example("simple_verbose").run().unwrap();
    let mut cmd = example.command();
    cmd.args(&["-vv"]).assert().failure();
}
