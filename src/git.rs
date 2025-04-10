use std::path::Path;
use std::process::Command;

/// Try to get the remote URL from a Git repository using git command
pub fn get_git_remote(path: &Path) -> Option<String> {
    let output = Command::new("git")
        .current_dir(path)
        .args(["remote", "get-url", "origin"])
        .output()
        .ok()?;

    if output.status.success() {
        let url = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !url.is_empty() {
            return Some(url);
        }
    }
    None
}