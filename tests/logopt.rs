use assert_cmd::prelude::*;
use escargot::CargoBuild;

#[test]
fn test_logopt_1() {
    let example = CargoBuild::new().example("logopt").run().unwrap();
    let mut cmd = example.command();
    let output = cmd.unwrap();
    output.clone().assert().success();
    output.assert().stdout("INFO\n");
}

#[test]
fn test_logopt_2() {
    let example = CargoBuild::new().example("logopt").run().unwrap();
    let mut cmd = example.command();
    let output = cmd.args(&["-L", "WARN"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("WARN\n");
}

#[test]
fn test_logopt_3() {
    let example = CargoBuild::new().example("logopt").run().unwrap();
    let mut cmd = example.command();
    cmd.args(&["-L", "none"]).assert().failure();
}

#[test]
fn test_logopt_4() {
    let example = CargoBuild::new().example("logopt").run().unwrap();
    let mut cmd = example.command();
    cmd.args(&["-l"]).assert().failure();
}

#[cfg(feature = "simplelog")]
#[test]
fn test_logopt_simplelog_1() {
    let example = CargoBuild::new()
        .example("logopt_simplelog")
        .features("simplelog")
        .run()
        .unwrap();
    let mut cmd = example.command();
    let output = cmd.unwrap();
    output.clone().assert().success();
    output.assert().stdout(predicates::str::ends_with("INFO\n"));
}

#[cfg(feature = "simplelog")]
#[test]
fn test_logopt_simplelog_2() {
    let example = CargoBuild::new()
        .example("logopt_simplelog")
        .features("simplelog")
        .run()
        .unwrap();
    let mut cmd = example.command();
    let output = cmd.args(&["-L", "WARN"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("");
}

#[cfg(feature = "simplelog")]
#[test]
fn test_logopt_simplelog_3() {
    let example = CargoBuild::new()
        .example("logopt_simplelog")
        .features("simplelog")
        .run()
        .unwrap();
    let mut cmd = example.command();
    cmd.args(&["-L", "none"]).assert().failure();
}

#[cfg(feature = "simplelog")]
#[test]
fn test_logopt_simplelog_4() {
    let example = CargoBuild::new()
        .example("logopt_simplelog")
        .features("simplelog")
        .run()
        .unwrap();
    let mut cmd = example.command();
    cmd.args(&["-l", "WARN"]).assert().failure();
}

#[test]
fn test_logoptlower_1() {
    let example = CargoBuild::new().example("logoptlower").run().unwrap();
    let mut cmd = example.command();
    let output = cmd.unwrap();
    output.clone().assert().success();
    output.assert().stdout("INFO\n");
}

#[test]
fn test_logoptlower_2() {
    let example = CargoBuild::new().example("logoptlower").run().unwrap();
    let mut cmd = example.command();
    let output = cmd.args(&["-l", "WARN"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("WARN\n");
}

#[test]
fn test_logoptlower_3() {
    let example = CargoBuild::new().example("logoptlower").run().unwrap();
    let mut cmd = example.command();
    cmd.args(&["-l", "none"]).assert().failure();
}

#[test]
fn test_logoptlower_4() {
    let example = CargoBuild::new().example("logoptlower").run().unwrap();
    let mut cmd = example.command();
    cmd.args(&["-L"]).assert().failure();
}

#[test]
fn test_logopt_nodef_1() {
    let example = CargoBuild::new()
        .example("logopt_no_default")
        .run()
        .unwrap();
    let mut cmd = example.command();
    let output = cmd.unwrap();
    output.clone().assert().success();
    output.assert().stdout("DEBUG\n");
}

#[test]
fn test_logopt_nodef_2() {
    let example = CargoBuild::new()
        .example("logopt_no_default")
        .run()
        .unwrap();
    let mut cmd = example.command();
    let output = cmd.args(&["-L", "WARN"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("WARN\n");
}

#[test]
fn test_logopt_nodef_3() {
    let example = CargoBuild::new()
        .example("logopt_no_default")
        .run()
        .unwrap();
    let mut cmd = example.command();
    cmd.args(&["-L", "none"]).assert().failure();
}

#[test]
fn test_logopt_nodef_4() {
    let example = CargoBuild::new()
        .example("logopt_no_default")
        .run()
        .unwrap();
    let mut cmd = example.command();
    cmd.args(&["-l"]).assert().failure();
}

#[cfg(feature = "simplelog")]
#[test]
fn test_logopt_nodef_simplelog_1() {
    let example = CargoBuild::new()
        .example("logopt_no_default_simplelog")
        .features("simplelog")
        .run()
        .unwrap();
    let mut cmd = example.command();
    let output = cmd.unwrap();
    output.clone().assert().success();
    output
        .assert()
        .stdout(predicates::str::ends_with("DEBUG\n"));
}

#[cfg(feature = "simplelog")]
#[test]
fn test_logopt_nodef_simplelog_2() {
    let example = CargoBuild::new()
        .example("logopt_no_default_simplelog")
        .features("simplelog")
        .run()
        .unwrap();
    let mut cmd = example.command();
    let output = cmd.args(&["-L", "WARN"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("");
}

#[cfg(feature = "simplelog")]
#[test]
fn test_logopt_nodef_simplelog_3() {
    let example = CargoBuild::new()
        .example("logopt_no_default_simplelog")
        .features("simplelog")
        .run()
        .unwrap();
    let mut cmd = example.command();
    cmd.args(&["-L", "none"]).assert().failure();
}

#[cfg(feature = "simplelog")]
#[test]
fn test_logopt_nodef_simplelog_4() {
    let example = CargoBuild::new()
        .example("logopt_no_default_simplelog")
        .features("simplelog")
        .run()
        .unwrap();
    let mut cmd = example.command();
    cmd.args(&["-l", "WARN"]).assert().failure();
}

#[test]
fn test_logoptlower_nodef_1() {
    let example = CargoBuild::new()
        .example("logoptlower_no_default")
        .run()
        .unwrap();
    let mut cmd = example.command();
    let output = cmd.unwrap();
    output.clone().assert().success();
    output.assert().stdout("ERROR\n");
}

#[test]
fn test_logoptlower_nodef_2() {
    let example = CargoBuild::new()
        .example("logoptlower_no_default")
        .run()
        .unwrap();
    let mut cmd = example.command();
    let output = cmd.args(&["-l", "WARN"]).unwrap();
    output.clone().assert().success();
    output.assert().stdout("WARN\n");
}

#[test]
fn test_logoptlower_nodef_3() {
    let example = CargoBuild::new()
        .example("logoptlower_no_default")
        .run()
        .unwrap();
    let mut cmd = example.command();
    cmd.args(&["-l", "none"]).assert().failure();
}

#[test]
fn test_logoptlower_nodef_4() {
    let example = CargoBuild::new()
        .example("logoptlower_no_default")
        .run()
        .unwrap();
    let mut cmd = example.command();
    cmd.args(&["-L"]).assert().failure();
}
