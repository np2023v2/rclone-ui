use rclone_ui::rclone::{FileItem, RcloneClient, Remote};

#[test]
fn test_remote_creation() {
    let remote = Remote {
        name: "test-remote".to_string(),
        remote_type: "s3".to_string(),
    };

    assert_eq!(remote.name, "test-remote");
    assert_eq!(remote.remote_type, "s3");
}

#[test]
fn test_file_item_creation() {
    let file = FileItem {
        path: "documents/test.txt".to_string(),
        name: "test.txt".to_string(),
        size: 1024,
        mime_type: "text/plain".to_string(),
        mod_time: "2024-01-01T00:00:00Z".to_string(),
        is_dir: false,
    };

    assert_eq!(file.name, "test.txt");
    assert_eq!(file.size, 1024);
    assert!(!file.is_dir);
}

#[test]
fn test_directory_item_creation() {
    let dir = FileItem {
        path: "documents".to_string(),
        name: "documents".to_string(),
        size: 0,
        mime_type: "inode/directory".to_string(),
        mod_time: "2024-01-01T00:00:00Z".to_string(),
        is_dir: true,
    };

    assert_eq!(dir.name, "documents");
    assert!(dir.is_dir);
}

#[test]
fn test_rclone_client_creation() {
    let client = RcloneClient::new();

    // Just verify we can create a client
    // Actual rclone operations require rclone to be installed
    let debug_str = format!("{:?}", client);
    assert!(debug_str.contains("RcloneClient"));
}

#[tokio::test]
async fn test_rclone_version_check() {
    let client = RcloneClient::new();

    // This test will pass if rclone is installed, skip if not
    match client.check_rclone() {
        Ok(version) => {
            assert!(!version.is_empty());
            println!("Rclone version: {}", version);
        }
        Err(_) => {
            println!("Rclone not installed, skipping test");
        }
    }
}
