use anyhow::{Context, Result};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

pub fn install() -> Result<()> {
    let hook_dir = Path::new(".git/hooks");
    if !hook_dir.exists() {
        anyhow::bail!("No .git/hooks directory found. Are you in the root of a git repository?");
    }

    let pre_commit_path = hook_dir.join("pre-commit");

    let script = r#"#!/bin/sh
# Raker pre-commit hook
# This intercepts commits to verify them against organizational context

# Check if raker is available
if ! command -v raker >/dev/null 2>&1; then
  echo "raker command not found. Please install Raker or add it to your PATH."
  exit 1
fi

echo "Raker: Analyzing staged changes..."

# We review the current directory using --diff and --strict
raker review --diff --strict
if [ $? -ne 0 ]; then
  echo "Raker: Commit rejected due to context violations. See details above."
  exit 1
fi

exit 0
"#;

    fs::write(&pre_commit_path, script).context("Failed to write pre-commit hook")?;

    let mut perms = fs::metadata(&pre_commit_path)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&pre_commit_path, perms).context("Failed to make hook executable")?;

    println!("Successfully installed Raker pre-commit hook at .git/hooks/pre-commit");
    Ok(())
}
