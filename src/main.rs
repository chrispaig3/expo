mod auth;
mod error;
mod gh;

use auth::AuthChecker;
use error::Result;
use gh::{GitHubClient, Visibility};
use futures::future::join_all;

enum Commands {
    Delete {
        repos: Vec<String>,
        yes: bool,
    },
    Visibility {
        visibility: Visibility,
        repos: Vec<String>,
    },
    Archive {
        repos: Vec<String>,
        unarchive: bool,
    },
    Create {
        repos: Vec<String>,
        public: bool,
        descriptions: Option<Vec<String>>,
    },
}

fn print_usage() {
    eprintln!("expo v{}", env!("CARGO_PKG_VERSION"));
    eprintln!("\nUsage: expo <COMMAND>\n");
    eprintln!("Commands:");
    eprintln!("  delete <REPOS>... [--yes]        Delete repositories");
    eprintln!("  visibility <public|private> <REPOS>...  Change repository visibility");
    eprintln!("  archive <REPOS>... [--unarchive] Archive or unarchive repositories");
    eprintln!("  create <REPOS>... [--public] [--description <DESC>]...  Create repositories");
    eprintln!("\nOptions:");
    eprintln!("  -h, --help     Print help");
    eprintln!("  -v, --version  Print version");
}

fn exit_with_error(msg: &str) -> ! {
    eprintln!("Error: {}", msg);
    std::process::exit(1);
}

fn parse_bool_flag_and_repos(args: &[String], flag: &str) -> (bool, Vec<String>) {
    let mut flag_set = false;
    let mut repos = Vec::new();

    for arg in args {
        if arg == flag {
            flag_set = true;
        } else if !arg.starts_with('-') {
            repos.push(arg.clone());
        }
    }

    (flag_set, repos)
}

fn require_repos(repos: Vec<String>, command: &str) -> Result<Vec<String>> {
    if repos.is_empty() {
        exit_with_error(&format!("{} requires at least one repository", command));
    }
    Ok(repos)
}

fn parse_args() -> Result<Commands> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }

    match args[1].as_str() {
        "-h" | "--help" => {
            print_usage();
            std::process::exit(0);
        }
        "-v" | "--version" => {
            println!("expo v{}", env!("CARGO_PKG_VERSION"));
            std::process::exit(0);
        }
        "delete" => {
            let (yes, repos) = parse_bool_flag_and_repos(&args[2..], "--yes");
            Ok(Commands::Delete { repos: require_repos(repos, "delete")?, yes })
        }
        "visibility" => {
            if args.len() < 4 {
                exit_with_error("visibility requires a visibility state and at least one repository");
            }

            let visibility = match args[2].as_str() {
                "public" => Visibility::Public,
                "private" => Visibility::Private,
                _ => exit_with_error("visibility must be 'public' or 'private'"),
            };

            let repos: Vec<String> = args[3..].iter().cloned().collect();
            Ok(Commands::Visibility { visibility, repos })
        }
        "archive" => {
            let (unarchive, repos) = parse_bool_flag_and_repos(&args[2..], "--unarchive");
            Ok(Commands::Archive { repos: require_repos(repos, "archive")?, unarchive })
        }
        "create" => {
            let mut public = false;
            let mut descriptions = Vec::new();
            let mut repos = Vec::new();
            let mut i = 2;

            while i < args.len() {
                match args[i].as_str() {
                    "--public" => {
                        public = true;
                        i += 1;
                    }
                    "--description" => {
                        if i + 1 < args.len() {
                            descriptions.push(args[i + 1].clone());
                            i += 2;
                        } else {
                            exit_with_error("--description requires a value");
                        }
                    }
                    arg if !arg.starts_with('-') => {
                        repos.push(arg.to_string());
                        i += 1;
                    }
                    _ => exit_with_error(&format!("unknown option '{}'", args[i])),
                }
            }

            Ok(Commands::Create { repos: require_repos(repos, "create")?, public, descriptions: if descriptions.is_empty() { None } else { Some(descriptions) } })
        }
        _ => {
            eprintln!("Error: unknown command '{}'", args[1]);
            print_usage();
            std::process::exit(1);
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
    let command = parse_args()?;
    AuthChecker::new().verify_authentication()?;
    let gh = GitHubClient::new();

    match command {
        Commands::Delete { repos, yes } => {
            execute_concurrent(repos, gh, move |gh, repo| async move {
                gh.delete_repository(&repo, !yes).await
            }).await;
        }
        Commands::Visibility { repos, visibility } => {
            execute_concurrent(repos, gh, move |gh, repo| {
                let vis = visibility.clone();
                async move { gh.change_visibility(&repo, vis).await }
            }).await;
        }
        Commands::Archive { repos, unarchive } => {
            execute_concurrent(repos, gh, move |gh, repo| async move {
                gh.archive_repository(&repo, !unarchive).await
            }).await;
        }
        Commands::Create { repos, public, descriptions } => {
            for (index, repo) in repos.into_iter().enumerate() {
                let desc = descriptions.as_ref().and_then(|descs| descs.get(index)).map(|s| s.as_str());
                gh.create_repository(&repo, public, desc).await?;
            }
        }
    }

    Ok(())
}
