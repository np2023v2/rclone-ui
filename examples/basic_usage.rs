use rclone_ui::rclone::RcloneClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize rclone client
    let client = RcloneClient::new();

    // Check if rclone is available
    match client.check_rclone() {
        Ok(version) => println!("Rclone version: {}", version),
        Err(e) => {
            eprintln!("Error: rclone not found - {}", e);
            eprintln!("Please install rclone: https://rclone.org/install/");
            return Ok(());
        }
    }

    // List all configured remotes
    println!("\nConfigured remotes:");
    match client.list_remotes().await {
        Ok(remotes) => {
            if remotes.is_empty() {
                println!("  No remotes configured.");
                println!("  Run 'rclone config' to configure remotes.");
            } else {
                for remote in &remotes {
                    println!("  - {}", remote.name);
                }

                // If we have remotes, try to list files from the first one
                if let Some(first_remote) = remotes.first() {
                    println!("\nFiles in {}:", first_remote.name);
                    match client.list_files(&first_remote.name, "").await {
                        Ok(files) => {
                            if files.is_empty() {
                                println!("  (empty)");
                            } else {
                                for file in files.iter().take(10) {
                                    let icon = if file.is_dir { "📁" } else { "📄" };
                                    println!("  {} {}", icon, file.name);
                                }
                                if files.len() > 10 {
                                    println!("  ... and {} more items", files.len() - 10);
                                }
                            }
                        }
                        Err(e) => eprintln!("  Error listing files: {}", e),
                    }
                }
            }
        }
        Err(e) => eprintln!("Error listing remotes: {}", e),
    }

    Ok(())
}
