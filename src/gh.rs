use crate::error::{ExpoError, Result};
use std::process::{Command, Output, Stdio};

#[derive(Clone)]
pub enum Visibility {
    Public,
    Private,
}

#[derive(Clone)]
pub struct GitHubClient;

impl GitHubClient {
    pub fn new() -> Self {
        Self
    }

    async fn execute_gh_command(&self, args: &[&str]) -> Result<Output> {
        let args_owned: Vec<String> = args.iter().map(|s| s.to_string()).collect();
        
        tokio::task::spawn_blocking(move || {
            Command::new("gh")
                .args(&args_owned)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .map_err(|_| ExpoError::CommandExecution)
        })
        .await
        .map_err(|_| ExpoError::CommandExecution)?
    }

    fn handle_gh_response(&self, output: Output, repo: &str) -> Result<()> {
        if output.status.success() {
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if stderr.contains("Not Found") {
                Err(ExpoError::RepositoryNotFound(repo.to_string()))
            } else {
                Err(ExpoError::GitHubCommandFailed)
            }
        }
    }

    pub async fn delete_repository(&self, repo: &str, dry_run: bool) -> Result<()> {
        self.validate_repo_format(repo)?;

        if dry_run {
            println!("Dry run: not deleting repository {}. Use --yes to actually delete.", repo);
            return Ok(());
        }

        let endpoint = format!("repos/{}", repo);
        let args = ["api", "-X", "DELETE", &endpoint];
        let output = self.execute_gh_command(&args).await?;
        self.handle_gh_response(output, repo)?;

        println!("Repository {} deleted.", repo);
        Ok(())
    }

    pub async fn change_visibility(&self, repo: &str, visibility: Visibility) -> Result<()> {
        self.validate_repo_format(repo)?;

        let visibility_str = match visibility {
            Visibility::Public => "false",
            Visibility::Private => "true",
        };

        let endpoint = format!("repos/{}", repo);
        let private_arg = format!("private={}", visibility_str);
        let args = ["api", "-X", "PATCH", &endpoint, "-f", &private_arg];

        let output = self.execute_gh_command(&args).await?;
        self.handle_gh_response(output, repo)?;

        let vis_name = match visibility {
            Visibility::Public => "Public",
            Visibility::Private => "Private",
        };
        println!("Repository {} visibility changed to {}.", repo, vis_name);
        Ok(())
    }

    pub async fn archive_repository(&self, repo: &str, archive: bool) -> Result<()> {
        self.validate_repo_format(repo)?;

        let endpoint = format!("repos/{}", repo);
        let archived_arg = format!("archived={}", archive);
        let args = ["api", "-X", "PATCH", &endpoint, "-f", &archived_arg];

        let output = self.execute_gh_command(&args).await?;
        self.handle_gh_response(output, repo)?;

        let action = if archive { "archived" } else { "unarchived" };
        println!("Repository {} {}.", repo, action);
        Ok(())
    }

    pub async fn create_repository(&self, repo: &str, public: bool, description: Option<&str>) -> Result<()> {
        self.validate_repo_format(repo)?;

        let repo_name = repo.split('/').nth(1).unwrap();
        let name_arg = format!("name={}", repo_name);
        let private_arg = format!("private={}", !public);
        let endpoint = "user/repos";

        let mut args = vec!["api", "-X", "POST", endpoint, "-f", &name_arg, "-f", &private_arg];

        let desc_arg;
        if let Some(desc) = description {
            desc_arg = format!("description={}", desc);
            args.push("-f");
            args.push(&desc_arg);
        }

        let output = self.execute_gh_command(&args).await?;
        
        if output.status.success() {
            println!("Repository {} created successfully.", repo);
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if stderr.contains("already exists") {
                eprintln!("Repository {} already exists", repo);
            } else {
                eprintln!("Failed to create repository {}: {}", repo, stderr);
            }
            Err(ExpoError::GitHubCommandFailed)
        }
    }

    fn validate_repo_format(&self, repo: &str) -> Result<()> {
        if !repo.contains('/') || repo.split('/').count() != 2 {
            return Err(ExpoError::InvalidRepository(repo.to_string()));
        }
        Ok(())
    }
}
