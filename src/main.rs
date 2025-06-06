use clap::Parser;
use std::process::Command;

type Repo = String;

#[derive(Parser)]
#[command(version)]
struct Cli {
    repos: Vec<Repo>,

    #[arg(long)]
    yes: bool,
}


trait PluginManager {
    fn register(&self, plugin: Plugin);
    fn unregister(&self, plugin: Plugin);
    fn run(&self);
}

struct Plugin<'s> {
    name: &'s str,
    description: &'s str,
    version: &'s str,
    binary_path: &'s str,
}

impl PluginManager for Plugin<'_> {
    fn register(&self, plugin: Plugin) {
        todo!();
    }

    fn unregister(&self, plugin: Plugin) {
        todo!();
    }

    fn run(&self) {
       todo!() 
    }
}

impl<'s> Plugin<'s> {
    // A plugin in expo is simply a binary with a manifest.
    // A manifest will be a plain text file with the following format:
    // name: <plugin_name>
    // description: <plugin_description>
    // version: <plugin_version>
    // binary_path: <path_to_binary>
    fn new(name: &'s str, description: &'s str, version: &'s str, binary_path: &'s str) -> Self {
        Self {
            name,
            description,
            version,
            binary_path,
        }
    }
}

enum  AuthStatus {
    Authenticated,
    Failed,
}

fn check_auth_status() -> AuthStatus {
    let mut auth_check_cmd = Command::new("gh");
    auth_check_cmd.args(["auth", "status", "-h", "github.com"]);

    let status = auth_check_cmd.status();
    match status {
        Ok(status) if status.success() => AuthStatus::Authenticated,
        Ok(_) => AuthStatus::Failed,
        Err(_) => AuthStatus::Failed,
    }
}

fn main() {
    let cli = Cli::parse();

    // Auth check
    match check_auth_status() {
        AuthStatus::Authenticated => {
            println!("Authentication successful.");
        }
        AuthStatus::Failed => {
            eprintln!("Failed to authenticate with GitHub.");
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
            println!(
                "Dry run: not deleting repository {}. Use --yes to actually delete.",
                repo
            );
        }
    }
}
