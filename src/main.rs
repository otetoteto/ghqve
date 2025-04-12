use anyhow::{Context, Result, anyhow};
use clap::Parser;
use std::fs;
use std::path::PathBuf;

// ライブラリ内のモジュールを使用
use ghqve::args::Args;
use ghqve::git::get_git_remote;
use ghqve::path::{
    check_executable_path_conflict, check_ghq_root_conflict, check_target_path_conflict,
    get_ghq_root, parse_remote_url,
};

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
        return Err(anyhow!(
            "Project path does not exist: {}",
            project_path.display()
        ));
    }

    // 実行ファイルとの競合チェック
    check_executable_path_conflict(&project_path)?;

    // Try to get the Git remote URL if not provided
    let remote_url = if let Some(remote) = args.remote {
        remote
    } else {
        match get_git_remote(&project_path) {
            Some(url) => url,
            None if args.force => {
                eprintln!(
                    "Warning: Could not detect Git remote URL. Using project directory name."
                );
                format!(
                    "unknown/{}",
                    project_path.file_name().unwrap().to_string_lossy()
                )
            }
            None => {
                return Err(anyhow!(
                    "Could not detect Git remote URL. Use --remote or --force to override."
                ));
            }
        }
    };

    // Get ghq root
    let ghq_root = get_ghq_root()?;

    // ghqルートとの競合チェック
    check_ghq_root_conflict(&project_path, &ghq_root)?;

    // Determine target path based on remote URL
    let target_rel_path = parse_remote_url(&remote_url)?;
    let target_path = ghq_root.join(target_rel_path);

    // ターゲットパスとの競合チェック
    check_target_path_conflict(&project_path, &target_path)?;

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
        return Err(anyhow!(
            "Target path already exists: {}",
            target_path.display()
        ));
    }

    // Move the project
    fs::rename(&project_path, &target_path).context("Failed to move project")?;

    println!(
        "Successfully moved project to ghq management: {}",
        target_path.display()
    );
    Ok(())
}
