# ghqve

### Description

`ghqve` is a tool that helps you move existing Git projects into your [ghq](https://github.com/x-motemen/ghq) managed directory structure. It automatically detects the remote repository URL and relocates your project to the appropriate location in your ghq directory.

### Installation

```bash
cargo install --path .
```

Or install from the repository:

```bash
cargo install --git https://github.com/otetoteto/ghqve
```

### Prerequisites

- [ghq](https://github.com/x-motemen/ghq) must be installed and configured
- Git must be installed

### Usage

Basic usage:

```bash
ghqve /path/to/your/project
```

With options:

```bash
# Specify a remote URL manually
ghqve /path/to/your/project --remote https://github.com/username/repo

# Force move even if remote URL cannot be detected
ghqve /path/to/your/project --force

# Dry run (only show what would happen)
ghqve /path/to/your/project --dry-run
```

### Options

- `--remote, -r`: Specify a remote repository URL manually
- `--force, -f`: Force move even if remote URL cannot be detected
- `--dry-run, -d`: Show what would happen without actually moving the project

### How It Works

1. Detects the Git remote URL of your project (or uses the provided URL)
2. Converts the URL to a path according to ghq's directory structure
3. Moves your project to the appropriate location under your ghq root