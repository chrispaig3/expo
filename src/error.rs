use std::fmt;

#[derive(Debug)]
pub enum ExpoError {
    AuthenticationFailed,
    GitHubCommandFailed(String),
    RepositoryNotFound(String),
    InvalidRepository(String),
    CommandExecution(String),
}

impl fmt::Display for ExpoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExpoError::AuthenticationFailed => {
                write!(
                    f,
                    "Failed to authenticate with GitHub. Please run 'gh auth login'"
                )
            }
            ExpoError::GitHubCommandFailed(operation) => {
                write!(f, "GitHub operation failed: {}", operation)
            }
            ExpoError::RepositoryNotFound(repo) => {
                write!(f, "Repository '{}' not found or access denied", repo)
            }
            ExpoError::InvalidRepository(repo) => {
                write!(
                    f,
                    "Invalid repository format: '{}'. Expected format: owner/repo",
                    repo
                )
            }
            ExpoError::CommandExecution(cmd) => {
                write!(f, "Failed to execute command: {}", cmd)
            }
        }
    }
}

impl std::error::Error for ExpoError {}

pub type Result<T> = std::result::Result<T, ExpoError>;
