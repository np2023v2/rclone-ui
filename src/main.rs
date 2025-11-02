use clap::{Parser, Subcommand};
use rclone_ui::{rclone::RcloneClient, tui::App};

/// A TUI for rclone cloud file management
#[derive(Parser)]
#[command(name = "rclone-ui")]
#[command(about = "A terminal UI for rclone")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the interactive TUI (default)
    Tui,
    /// List all configured remotes
    Remotes,
    /// List files in a remote
    List {
        /// Remote name (e.g., "myremote:")
        remote: String,
        /// Optional path within the remote
        #[arg(default_value = "")]
        path: String,
    },
    /// Check rclone version
    Version,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    let client = RcloneClient::new();

    match cli.command {
        Some(Commands::Tui) | None => {
            // Default to TUI mode
            let mut app = App::new(client);
            app.run().await?;
        }
        Some(Commands::Remotes) => {
            let remotes = client.list_remotes().await?;
            if remotes.is_empty() {
                println!("No remotes configured.");
                println!("Run 'rclone config' to configure remotes.");
            } else {
                println!("Configured remotes:");
                for remote in remotes {
                    println!("  - {}", remote.name);
                }
            }
        }
        Some(Commands::List { remote, path }) => {
            let files = client.list_files(&remote, &path).await?;
            if files.is_empty() {
                println!("No files found.");
            } else {
                for file in files {
                    let icon = if file.is_dir { "📁" } else { "📄" };
                    println!("{} {}", icon, file.name);
                }
            }
        }
        Some(Commands::Version) => match client.check_rclone() {
            Ok(version) => println!("{}", version),
            Err(e) => eprintln!("Error: {}", e),
        },
    }

    Ok(())
}
