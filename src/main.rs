mod auth;
mod error;
mod gh;

use auth::AuthChecker;
use clap::{Parser, Subcommand};
use error::Result;
use gh::{GitHubClient, Visibility};

type Repo = String;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Delete {
        repos: Vec<Repo>,
        #[arg(long)]
        yes: bool,
    },
    Visibility {
        repos: Vec<Repo>,
        #[arg(value_enum)]
        visibility: VisState,
    },
    Archive {
        repos: Vec<Repo>,
        #[arg(long)]
        unarchive: bool,
    },
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum VisState {
    Public,
    Private,
}

impl From<VisState> for Visibility {
    fn from(state: VisState) -> Self {
        match state {
            VisState::Public => Visibility::Public,
            VisState::Private => Visibility::Private,
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let auth_checker = AuthChecker::new();
    let github_client = GitHubClient::new();

    auth_checker.verify_authentication()?;

    match cli.command {
        Commands::Delete { repos, yes } => {
            for repo in repos {
                github_client.delete_repository(&repo, !yes)?;
            }
        }
        Commands::Visibility { repos, visibility } => {
            for repo in repos {
                github_client.change_visibility(&repo, visibility.clone().into())?;
            }
        }
        Commands::Archive { repos, unarchive } => {
            for repo in repos {
                github_client.archive_repository(&repo, !unarchive)?;
            }
        }
    }

    Ok(())
}
