use anyhow::{anyhow, Context, Result};
use regex::Regex;
use std::path::PathBuf;
use std::process::Command;
use url::Url;

/// Get the primary ghq root directory
pub fn get_ghq_root() -> Result<PathBuf> {
    let output = Command::new("ghq")
        .arg("root")
        .output()
        .context("Failed to execute ghq root command")?;

    if !output.status.success() {
        return Err(anyhow!(
            "ghq root command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let root = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(PathBuf::from(root))
}

/// Parse a remote URL into a relative path according to ghq's directory structure
pub fn parse_remote_url(remote_url: &str) -> Result<PathBuf> {
    // Try to parse as a URL
    if let Ok(url) = Url::parse(remote_url) {
        let host = url.host_str().ok_or_else(|| anyhow!("Invalid URL host"))?;

        // Remove .git extension if present
        let path = url.path().trim_start_matches('/').trim_end_matches(".git");
        return Ok(PathBuf::from(host).join(path));
    }

    // Try to parse as a GitHub shorthand (user/repo or github.com/user/repo)
    let re = Regex::new(r"^(?:(?:github\.com|gitlab\.com)/)?([^/]+/[^/]+)$").unwrap();
    if let Some(caps) = re.captures(remote_url) {
        let path = caps.get(1).unwrap().as_str();
        return Ok(PathBuf::from("github.com").join(path));
    }

    // Handle "unknown/dir" format from --force
    if remote_url.starts_with("unknown/") {
        return Ok(PathBuf::from(remote_url));
    }

    Err(anyhow!("Could not parse remote URL: {}", remote_url))
}