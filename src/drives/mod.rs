pub mod detector;

use std::time::Duration;
use tokio::sync::mpsc;
use crate::state::models::DriveInfo;

pub use detector::{classify_drive, derive_drive_label, format_bytes, is_system_mount, list_drives};

/// Background monitor that periodically polls for connected / disconnected storage volumes.
pub struct DriveMonitor {
    poll_interval: Duration,
}

impl DriveMonitor {
    pub fn new(poll_interval: Duration) -> Self {
        Self { poll_interval }
    }

    /// Spawns a background task that periodically scans for drives and emits the updated list
    /// whenever the set of detected drives changes.
    pub fn start(self) -> mpsc::UnboundedReceiver<Vec<DriveInfo>> {
        let (tx, rx) = mpsc::unbounded_channel();

        tokio::spawn(async move {
            let mut previous_ids: Vec<String> = Vec::new();

            loop {
                let current_drives = list_drives();
                let current_ids: Vec<String> = current_drives.iter().map(|d| d.id.clone()).collect();

                // If drive set changed or this is the initial scan, emit update
                if current_ids != previous_ids {
                    previous_ids = current_ids;
                    if tx.send(current_drives).is_err() {
                        // Receiver dropped, stop background loop
                        break;
                    }
                }

                tokio::time::sleep(self.poll_interval).await;
            }
        });

        rx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_drive_monitor_emits_initial_drives() {
        let monitor = DriveMonitor::new(Duration::from_millis(50));
        let mut rx = monitor.start();

        let initial_drives = tokio::time::timeout(Duration::from_secs(2), rx.recv())
            .await
            .expect("Timed out waiting for initial drive scan")
            .expect("Channel closed unexpectedly");

        assert!(!initial_drives.is_empty(), "Expected at least one drive in initial emission");
    }
}
