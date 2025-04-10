use clap::Parser;

/// Move an existing project to ghq management
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to the project to move
    #[arg(required = true)]
    pub project_path: String,

    /// Remote repository URL (optional)
    #[arg(short, long)]
    pub remote: Option<String>,

    /// Force move even if remote URL cannot be detected
    #[arg(short, long)]
    pub force: bool,

    /// Only print the destination path without actually moving
    #[arg(short, long)]
    pub dry_run: bool,
}