use std::fs;

use anyhow::Result;
use assert_cmd::Command;

const FRONTMATTER: &str = "tests/inputs/front_matter_01.md";

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
fn front_matter1() -> Result<()> {
    run(&[FRONTMATTER], "tests/expected/front_matter_01_expected.md")
}
