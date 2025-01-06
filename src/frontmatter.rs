use anyhow::Result;

pub fn remove_frontmatter(file: &str) -> Result<String> {
    Ok(file.replace("---", "").replace("---", ""))
}
