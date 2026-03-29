mod auth;
mod error;
mod gh;

use auth::AuthChecker;
use clap::{Parser, Subcommand};
use error::Result;
use gh::{GitHubClient, Visibility};
use futures::future::join_all;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Delete {
        repos: Vec<String>,
        #[arg(long)]
        yes: bool,
    },
    Visibility {
        #[arg(value_enum)]
        visibility: VisState,
        repos: Vec<String>,
    },
    Archive {
        repos: Vec<String>,
        #[arg(long)]
        unarchive: bool,
    },
    Create {
        repos: Vec<String>,
        #[arg(long)]
        public: bool,
        #[arg(long)]
        description: Option<String>,
    },
}

#[derive(clap::ValueEnum, Clone)]
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

async fn execute_concurrent<F, Fut>(repos: Vec<String>, gh: GitHubClient, f: F)
where
    F: Fn(GitHubClient, String) -> Fut,
    Fut: std::future::Future<Output = Result<()>> + Send + 'static,
{
    let tasks: Vec<_> = repos
        .into_iter()
        .map(|repo| {
            let gh = gh.clone();
            tokio::spawn(f(gh, repo))
        })
        .collect();

    let results = join_all(tasks).await;
    for result in results {
        if let Err(e) = result {
            eprintln!("Task error: {}", e);
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    AuthChecker::new().verify_authentication()?;
    let gh = GitHubClient::new();

    match cli.command {
        Commands::Delete { repos, yes } => {
            execute_concurrent(repos, gh, move |gh, repo| async move {
                gh.delete_repository(&repo, !yes).await
            }).await;
        }
        Commands::Visibility { repos, visibility } => {
            let vis: Visibility = visibility.into();
            execute_concurrent(repos, gh, move |gh, repo| {
                let vis = vis.clone();
                async move { gh.change_visibility(&repo, vis).await }
            }).await;
        }
        Commands::Archive { repos, unarchive } => {
            execute_concurrent(repos, gh, move |gh, repo| async move {
                gh.archive_repository(&repo, !unarchive).await
            }).await;
        }
        Commands::Create { repos, public, description } => {
            execute_concurrent(repos, gh, move |gh, repo| {
                let desc = description.clone();
                async move { gh.create_repository(&repo, public, desc.as_deref()).await }
            }).await;
        }
    }

    Ok(())
}
