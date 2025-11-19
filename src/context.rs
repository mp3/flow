use anyhow::Result;
use git2::Repository;
use std::env;
use std::path::PathBuf;

pub struct ContextManager;

impl ContextManager {
    pub fn get_context() -> Result<PathBuf> {
        let current_dir = env::current_dir()?;
        
        // Try to find a git repository starting from current_dir
        match Repository::discover(&current_dir) {
            Ok(repo) => {
                // If found, return the workdir (root of the repo)
                // workdir() returns Option<&Path>, usually Some for non-bare repos
                Ok(repo.workdir().unwrap_or(&current_dir).to_path_buf())
            }
            Err(_) => {
                // If not a git repo, just return the current directory
                // Or we could return a "global" context path if we wanted
                Ok(current_dir)
            }
        }
    }
}
