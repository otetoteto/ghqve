use anyhow::{anyhow, Result, Context};
use std::fs;
use std::path::PathBuf;
use clap::Parser;

// ライブラリ内のモジュールを使用
use ghqve::args::Args;
use ghqve::git::get_git_remote;
use ghqve::path::{get_ghq_root, parse_remote_url};

fn main() -> Result<()> {
    let args = Args::parse();

    // Convert to absolute path
    let project_path = PathBuf::from(args.project_path);
    let project_path = if project_path.is_absolute() {
        project_path
    } else {
        std::env::current_dir()?.join(project_path)
    };

    if !project_path.exists() {
        return Err(anyhow!("Project path does not exist: {}", project_path.display()));
    }

    // Get current executable path
    let current_exe = std::env::current_exe().ok();

    // Check if we're trying to move the directory containing the current executable
    if let Some(exe_path) = current_exe {
        if let Some(exe_parent) = exe_path.parent() {
            if let Ok(exe_dir) = exe_parent.canonicalize() {
                if let Ok(project_canonical) = project_path.canonicalize() {
                    if exe_dir.starts_with(&project_canonical) || project_canonical.starts_with(&exe_dir) {
                        return Err(anyhow!("Cannot move the directory containing the currently running executable. Run this tool from a different directory."));
                    }
                }
            }
        }
    }

    // Try to get the Git remote URL if not provided
    let remote_url = if let Some(remote) = args.remote {
        remote
    } else {
        match get_git_remote(&project_path) {
            Some(url) => url,
            None if args.force => {
                eprintln!("Warning: Could not detect Git remote URL. Using project directory name.");
                format!("unknown/{}", project_path.file_name().unwrap().to_string_lossy())
            }
            None => {
                return Err(anyhow!("Could not detect Git remote URL. Use --remote or --force to override."));
            }
        }
    };

    // Get ghq root
    let ghq_root = get_ghq_root()?;

    // Check if project_path is the same as or contains ghq_root
    if let Ok(project_canonical) = project_path.canonicalize() {
        if let Ok(ghq_canonical) = ghq_root.canonicalize() {
            if project_canonical == ghq_canonical || ghq_canonical.starts_with(&project_canonical) {
                return Err(anyhow!("Cannot move ghq root directory or a directory that contains it."));
            }
        }
    }

    // Determine target path based on remote URL
    let target_rel_path = parse_remote_url(&remote_url)?;
    let target_path = ghq_root.join(target_rel_path);

    // Check if project_path is the same as target_path
    if let Ok(project_canonical) = project_path.canonicalize() {
        if let Ok(target_parent) = target_path.parent().map_or(Ok(target_path.clone()), |p| p.canonicalize()) {
            if project_canonical == target_parent {
                return Err(anyhow!("Source and target directories are the same."));
            }
        }
    }

    println!("Source path: {}", project_path.display());
    println!("Target path: {}", target_path.display());

    if args.dry_run {
        println!("Dry run - not moving files.");
        return Ok(());
    }

    // Ensure parent directory exists
    if let Some(parent) = target_path.parent() {
        fs::create_dir_all(parent).context("Failed to create target directory")?;
    }

    // Check if target directory already exists
    if target_path.exists() {
        return Err(anyhow!("Target path already exists: {}", target_path.display()));
    }

    // Move the project
    fs::rename(&project_path, &target_path).context("Failed to move project")?;

    println!("Successfully moved project to ghq management: {}", target_path.display());
    Ok(())
}
