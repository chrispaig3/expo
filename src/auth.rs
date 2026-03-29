use crate::error::{ExpoError, Result};
use std::process::{Command, Stdio};

pub struct AuthChecker;

impl Default for AuthChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthChecker {
    pub fn new() -> Self {
        Self
    }

    pub fn verify_authentication(&self) -> Result<()> {
        let output = Command::new("gh")
            .args(["auth", "status", "-h", "github.com"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map_err(|_| ExpoError::CommandExecution)?;

        if output.success() {
            println!("Authentication successful.");
            Ok(())
        } else {
            Err(ExpoError::AuthenticationFailed)
        }
    }
}