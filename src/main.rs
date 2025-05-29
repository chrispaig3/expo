use clap::Parser;
use std::process::Command;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    repo: String,

    #[arg(long)]
    yes: bool,
}

fn main() {
    let cli = Cli::parse();

    // Auth check
    let mut auth_check_cmd = Command::new("gh");
    auth_check_cmd.args(["auth", "status", "-h", "github.com"]);

    let auth_check_status = auth_check_cmd.status().expect("Failed to check GitHub authentication status");
    if !auth_check_status.success() {
        let mut auth_cmd = Command::new("gh");
        auth_cmd.args(["auth", "refresh", "-h", "github.com", "-s", "delete_repo"]);

        let auth_status = auth_cmd.status().expect("Failed to refresh GitHub authentication");
        if !auth_status.success() {
            eprintln!("Failed to refresh authentication. Please ensure you have the necessary permissions.");
            return;
        }
    }

    let mut cmd = Command::new("gh");
    cmd.args(["api", "-X", "DELETE", &format!("repos/{}", cli.repo)]);

    if cli.yes {
        let status = cmd.status().expect("Failed to execute gh command");
        if status.success() {
            println!("Repository {} deleted.", cli.repo);
        } else {
            eprintln!("Failed to delete repository.");
        }
    } else {
        println!("Dry run: not deleting. Use --yes to actually delete.");
    }
}