use clap::Parser;
use std::process::Command;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    repos: Vec<String>,

    #[arg(long)]
    yes: bool,
}

enum AuthStatus {
    Authenticated,
    NeedsRefresh,
    Failed,
}

fn check_auth_status() -> AuthStatus {
    let mut auth_check_cmd = Command::new("gh");
    auth_check_cmd.args(["auth", "status", "-h", "github.com"]);

    let auth_check_status = auth_check_cmd.status();
    match auth_check_status {
        Ok(status) if status.success() => AuthStatus::Authenticated,
        Ok(_) => AuthStatus::NeedsRefresh,
        Err(_) => AuthStatus::Failed,
    }
}

fn refresh_auth() -> bool {
    let mut auth_cmd = Command::new("gh");
    auth_cmd.args(["auth", "refresh", "-h", "github.com", "-s", "delete_repo"]);

    let auth_status = auth_cmd.status();
    auth_status.map_or(false, |status| status.success())
}

fn main() {
    let cli = Cli::parse();
    
    // Auth check
    match check_auth_status() {
        AuthStatus::Authenticated => {
            println!("Authentication successful.");
        }
        AuthStatus::NeedsRefresh => {
            println!("Authentication needs to be refreshed.");
            if !refresh_auth() {
                eprintln!("Failed to refresh authentication. Please ensure you have the necessary permissions.");
                return;
            }
        }
        AuthStatus::Failed => {
            eprintln!("Failed to check GitHub authentication status.");
            return;
        }
    }

    for repo in cli.repos {
        let mut cmd = Command::new("gh");
        cmd.args(["api", "-X", "DELETE", &format!("repos/{}", repo)]);

        if cli.yes {
            let status = cmd.status().expect("Failed to execute gh command");
            if status.success() {
                println!("Repository {} deleted.", repo);
            } else {
                eprintln!("Failed to delete repository {}.", repo);
            }
        } else {
            println!("Dry run: not deleting repository {}. Use --yes to actually delete.", repo);
        }
    }
}