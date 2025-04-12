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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args_with_project_path_only() {
        let args = Args::parse_from(["ghqve", "/path/to/project"]);
        assert_eq!(args.project_path, "/path/to/project");
        assert_eq!(args.remote, None);
        assert!(!args.force);
        assert!(!args.dry_run);
    }

    #[test]
    fn test_args_with_remote_url() {
        let args = Args::parse_from([
            "ghqve",
            "/path/to/project",
            "--remote",
            "https://github.com/user/repo",
        ]);
        assert_eq!(args.project_path, "/path/to/project");
        assert_eq!(
            args.remote,
            Some("https://github.com/user/repo".to_string())
        );
        assert!(!args.force);
        assert!(!args.dry_run);
    }

    #[test]
    fn test_args_with_force_flag() {
        let args = Args::parse_from(["ghqve", "/path/to/project", "--force"]);
        assert_eq!(args.project_path, "/path/to/project");
        assert_eq!(args.remote, None);
        assert!(args.force);
        assert!(!args.dry_run);
    }

    #[test]
    fn test_args_with_dry_run_flag() {
        let args = Args::parse_from(["ghqve", "/path/to/project", "--dry-run"]);
        assert_eq!(args.project_path, "/path/to/project");
        assert_eq!(args.remote, None);
        assert!(!args.force);
        assert!(args.dry_run);
    }

    #[test]
    fn test_args_with_short_flags() {
        let args = Args::parse_from([
            "ghqve",
            "/path/to/project",
            "-r",
            "https://example.com/repo",
            "-f",
            "-d",
        ]);
        assert_eq!(args.project_path, "/path/to/project");
        assert_eq!(args.remote, Some("https://example.com/repo".to_string()));
        assert!(args.force);
        assert!(args.dry_run);
    }
}
