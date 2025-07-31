use crate::error::{ExpoError, Result};
use std::process::{Command, Output, Stdio};

#[derive(Clone, Debug)]
pub enum Visibility {
    Public,
    Private,
}

pub struct GitHubClient;

impl GitHubClient {
    pub fn new() -> Self {
        Self
    }

    fn execute_gh_command(&self, args: &[&str]) -> Result<Output> {
        let output = Command::new("gh")
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|_| ExpoError::CommandExecution("gh".to_string()))?;

        Ok(output)
    }

    fn handle_gh_response(&self, output: Output, operation: &str, repo: &str) -> Result<()> {
        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if stderr.contains("Not Found") {
                Err(ExpoError::RepositoryNotFound(repo.to_string()))
            } else {
                Err(ExpoError::GitHubCommandFailed(format!(
                    "{} repository {}",
                    operation, repo
                )))
            }
        }
    }

    pub fn delete_repository(&self, repo: &str, dry_run: bool) -> Result<()> {
        self.validate_repo_format(repo)?;

        if dry_run {
            println!(
                "Dry run: not deleting repository {}. Use --yes to actually delete.",
                repo
            );
            return Ok(());
        }

        let args = ["api", "-X", "DELETE", &format!("repos/{}", repo)];
        let output = self.execute_gh_command(&args)?;
        self.handle_gh_response(output, "delete", repo)?;

        println!("Repository {} deleted.", repo);
        Ok(())
    }

    pub fn change_visibility(&self, repo: &str, visibility: Visibility) -> Result<()> {
        self.validate_repo_format(repo)?;

        let visibility_str = match visibility {
            Visibility::Public => "false",
            Visibility::Private => "true",
        };

        let args = [
            "api",
            "-X",
            "PATCH",
            &format!("repos/{}", repo),
            "-f",
            &format!("private={}", visibility_str),
        ];

        let output = self.execute_gh_command(&args)?;
        self.handle_gh_response(output, "change visibility for", repo)?;

        println!(
            "Repository {} visibility changed to {:?}.",
            repo, visibility
        );
        Ok(())
    }

    pub fn archive_repository(&self, repo: &str, archive: bool) -> Result<()> {
        self.validate_repo_format(repo)?;

        let args = [
            "api",
            "-X",
            "PATCH",
            &format!("repos/{}", repo),
            "-f",
            &format!("archived={}", archive),
        ];

        let output = self.execute_gh_command(&args)?;
        let action = if archive { "archive" } else { "unarchive" };
        self.handle_gh_response(output, action, repo)?;

        if archive {
            println!("Repository {} archived.", repo);
        } else {
            println!("Repository {} unarchived.", repo);
        }
        Ok(())
    }

    fn validate_repo_format(&self, repo: &str) -> Result<()> {
        if !repo.contains('/') || repo.split('/').count() != 2 {
            return Err(ExpoError::InvalidRepository(repo.to_string()));
        }
        Ok(())
    }
}
