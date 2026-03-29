mod auth;
mod error;
mod gh;

use auth::AuthChecker;
use clap::{Parser, Subcommand};
use error::Result;
use gh::{GitHubClient, Visibility};
use futures::future::join_all;

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
        #[arg(value_enum)]
        visibility: VisState,
        repos: Vec<Repo>,
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

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let check = AuthChecker::new();
    let gh = GitHubClient::new();

    check.verify_authentication()?;

    match cli.command {
        Commands::Delete { repos, yes } => {
            let tasks: Vec<_> = repos
                .into_iter()
                .map(|repo| {
                    let gh = gh.clone();
                    tokio::spawn(async move {
                        gh.delete_repository(&repo, !yes).await
                    })
                })
                .collect();

            let results = join_all(tasks).await;
            for result in results {
                if let Err(e) = result {
                    eprintln!("Task error: {}", e);
                }
            }
        }
        Commands::Visibility { repos, visibility } => {
            let vis: Visibility = visibility.clone().into();
            let tasks: Vec<_> = repos
                .into_iter()
                .map(|repo| {
                    let gh = gh.clone();
                    let vis = vis.clone();
                    tokio::spawn(async move {
                        gh.change_visibility(&repo, vis).await
                    })
                })
                .collect();

            let results = join_all(tasks).await;
            for result in results {
                if let Err(e) = result {
                    eprintln!("Task error: {}", e);
                }
            }
        }
        Commands::Archive { repos, unarchive } => {
            let tasks: Vec<_> = repos
                .into_iter()
                .map(|repo| {
                    let gh = gh.clone();
                    tokio::spawn(async move {
                        gh.archive_repository(&repo, !unarchive).await
                    })
                })
                .collect();

            let results = join_all(tasks).await;
            for result in results {
                if let Err(e) = result {
                    eprintln!("Task error: {}", e);
                }
            }
        }
    }

    Ok(())
}
