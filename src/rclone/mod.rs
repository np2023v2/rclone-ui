use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::process::{Command, Stdio};

/// Represents an rclone remote
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Remote {
    pub name: String,
    pub remote_type: String,
}

/// Represents a file or directory in rclone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileItem {
    pub path: String,
    pub name: String,
    pub size: i64,
    pub mime_type: String,
    pub mod_time: String,
    pub is_dir: bool,
}

/// Rclone client for interacting with rclone CLI
#[derive(Debug)]
pub struct RcloneClient {
    rclone_path: String,
}

impl Default for RcloneClient {
    fn default() -> Self {
        Self::new()
    }
}

impl RcloneClient {
    /// Create a new rclone client
    pub fn new() -> Self {
        Self {
            rclone_path: "rclone".to_string(),
        }
    }

    /// List all configured remotes
    pub async fn list_remotes(&self) -> Result<Vec<Remote>> {
        let output = Command::new(&self.rclone_path)
            .args(["listremotes"])
            .stdout(Stdio::piped())
            .output()
            .context("Failed to execute rclone listremotes")?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("rclone listremotes failed: {}", error);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let remotes: Vec<Remote> = stdout
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let name = line.trim_end_matches(':').to_string();
                Remote {
                    name: name.clone(),
                    remote_type: "unknown".to_string(),
                }
            })
            .collect();

        Ok(remotes)
    }

    /// List files in a remote path
    pub async fn list_files(&self, remote: &str, path: &str) -> Result<Vec<FileItem>> {
        let full_path = if path.is_empty() {
            remote.to_string()
        } else {
            format!("{}:{}", remote, path)
        };

        let output = Command::new(&self.rclone_path)
            .args(["lsjson", &full_path])
            .stdout(Stdio::piped())
            .output()
            .context("Failed to execute rclone lsjson")?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("rclone lsjson failed: {}", error);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);

        #[derive(Deserialize)]
        struct RcloneItem {
            #[serde(rename = "Path")]
            path: String,
            #[serde(rename = "Name")]
            name: String,
            #[serde(rename = "Size")]
            size: i64,
            #[serde(rename = "MimeType")]
            mime_type: String,
            #[serde(rename = "ModTime")]
            mod_time: String,
            #[serde(rename = "IsDir")]
            is_dir: bool,
        }

        let items: Vec<RcloneItem> =
            serde_json::from_str(&stdout).context("Failed to parse rclone lsjson output")?;

        let file_items: Vec<FileItem> = items
            .into_iter()
            .map(|item| FileItem {
                path: item.path,
                name: item.name,
                size: item.size,
                mime_type: item.mime_type,
                mod_time: item.mod_time,
                is_dir: item.is_dir,
            })
            .collect();

        Ok(file_items)
    }

    /// Copy a file from source to destination
    pub async fn copy(&self, source: &str, dest: &str) -> Result<()> {
        let output = Command::new(&self.rclone_path)
            .args(["copy", source, dest])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .context("Failed to execute rclone copy")?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("rclone copy failed: {}", error);
        }

        Ok(())
    }

    /// Delete a file or directory
    pub async fn delete(&self, path: &str) -> Result<()> {
        let output = Command::new(&self.rclone_path)
            .args(["delete", path])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .context("Failed to execute rclone delete")?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("rclone delete failed: {}", error);
        }

        Ok(())
    }

    /// Check if rclone is available
    pub fn check_rclone(&self) -> Result<String> {
        let output = Command::new(&self.rclone_path)
            .args(["version"])
            .stdout(Stdio::piped())
            .output()
            .context("Failed to execute rclone version")?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("rclone version failed: {}", error);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let version = stdout.lines().next().unwrap_or("unknown").to_string();

        Ok(version)
    }
}
