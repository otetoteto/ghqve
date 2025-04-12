use anyhow::{Context, Result, anyhow};
use regex::Regex;
use std::path::{Path, PathBuf};
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

/// 実行ファイルのディレクトリとプロジェクトパスの関係をチェック
pub fn check_executable_path_conflict(project_path: &Path) -> Result<()> {
    // 現在の実行ファイルのパスを取得
    let Some(exe_path) = std::env::current_exe().ok() else {
        return Ok(()); // 実行ファイルのパスが取得できなければ問題なし
    };

    let Some(exe_parent) = exe_path.parent() else {
        return Ok(()); // 親ディレクトリがなければ問題なし
    };

    // パスを正規化して比較
    let exe_dir = exe_parent.canonicalize().ok();
    let project_canonical = project_path.canonicalize().ok();

    match (exe_dir, project_canonical) {
        (Some(exe_dir), Some(project_canonical)) => {
            if exe_dir.starts_with(&project_canonical) || project_canonical.starts_with(&exe_dir) {
                return Err(anyhow!(
                    "Cannot move the directory containing the currently running executable. Run this tool from a different directory."
                ));
            }
        }
        _ => {} // いずれかのパスが正規化できない場合は問題なし
    }

    Ok(())
}

/// プロジェクトパスがghqルートディレクトリを含むかチェック
pub fn check_ghq_root_conflict(project_path: &Path, ghq_root: &Path) -> Result<()> {
    let project_canonical = project_path.canonicalize().ok();
    let ghq_canonical = ghq_root.canonicalize().ok();

    match (project_canonical, ghq_canonical) {
        (Some(project_canonical), Some(ghq_canonical)) => {
            if project_canonical == ghq_canonical || ghq_canonical.starts_with(&project_canonical) {
                return Err(anyhow!(
                    "Cannot move ghq root directory or a directory that contains it."
                ));
            }
        }
        _ => {} // いずれかのパスが正規化できない場合は問題なし
    }

    Ok(())
}

/// プロジェクトパスが移動先と同じかチェック
pub fn check_target_path_conflict(project_path: &Path, target_path: &Path) -> Result<()> {
    let project_canonical = project_path.canonicalize().ok();
    let target_parent = target_path
        .parent()
        .map(|p| p.canonicalize().ok())
        .flatten()
        .or_else(|| target_path.canonicalize().ok());

    match (project_canonical, target_parent) {
        (Some(project_canonical), Some(target_parent)) => {
            if project_canonical == target_parent {
                return Err(anyhow!("Source and target directories are the same."));
            }
        }
        _ => {} // いずれかのパスが正規化できない場合は問題なし
    }

    Ok(())
}
