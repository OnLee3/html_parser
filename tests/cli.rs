use anyhow::Result;
use assert_cmd::Command;
use std::fs;

const HEADING: &str = "tests/inputs/heading1.md";

fn run(args: &[&str], expected_file: &str) -> Result<()> {
    let expected = fs::read_to_string(expected_file)?;
    let output = Command::cargo_bin("html_parser")?
        .args(args)
        .output()
        .expect("fail");
    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, expected);
    Ok(())
}

#[test]
fn heading1() -> Result<()> {
    run(&[HEADING], "tests/expected/heading1_expected")
}
