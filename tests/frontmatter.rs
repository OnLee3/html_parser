use anyhow::Result;
use html_parser::remove_frontmatter;
//const FRONTMATTER: &str = "tests/inputs/front_matter1.md";

#[test]
fn front_matter1() -> Result<()> {
    assert_eq!(remove_frontmatter("---")?, "");
    Ok(())
}

#[test]
fn front_matter2() -> Result<()> {
    assert_eq!(remove_frontmatter("---\n---")?, "");
    Ok(())
}
