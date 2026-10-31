use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Type of storage drive detected by the operating system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DriveType {
    Internal,
    External,
    Network,
}

/// Metadata and status for an available disk volume.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DriveInfo {
    pub id: String,
    pub name: String,
    pub drive_type: DriveType,
    pub mount_path: PathBuf,
    pub size_formatted: String,
    pub used_percentage: u8,
    pub is_mounted: bool,
}

/// Represents an individual media asset cataloged on the external drive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MediaItem {
    pub id: i64,
    pub file_hash: String,
    pub original_relative_path: String,
    pub current_relative_path: String,
    pub file_size: i64,
    pub mime_type: String,
    pub duration_seconds: Option<i64>,
    pub metadata_json: Option<String>,
    pub album_id: Option<i64>,
    pub created_at: String,
}

/// Represents an album folder physically located within `albums/`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Album {
    pub id: i64,
    pub name: String,
    pub relative_path: String,
    pub description: Option<String>,
    pub media_count: i64,
    pub created_at: String,
    pub preview_item: Option<MediaItem>,
}

/// Category tag for organizing media.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub color_hex: String,
    pub category: String,
}

/// Real-time progress metrics emitted during a media discovery scan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ScanProgress {
    pub is_scanning: bool,
    pub is_paused: bool,
    pub is_stopped: bool,
    pub scanned_count: usize,
    pub found_count: usize,
    pub current_folder: String,
    pub active_ignore_rules: usize,
}

/// Filtering options for the media catalog.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum MediaFilterType {
    #[default]
    All,
    Images,
    Videos,
}

/// Sorting criteria for catalog queries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum MediaSortBy {
    #[default]
    Date,
    Name,
    Size,
}

/// Sort direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Asc,
    #[default]
    Desc,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drive_info_serialization() {
        let drive = DriveInfo {
            id: "disk1s1".to_string(),
            name: "SanDisk Extreme".to_string(),
            drive_type: DriveType::External,
            mount_path: PathBuf::from("/run/media/destiny/SanDisk"),
            size_formatted: "1.8 TB".to_string(),
            used_percentage: 42,
            is_mounted: true,
        };

        let json = serde_json::to_string(&drive).expect("Failed to serialize DriveInfo");
        let deserialized: DriveInfo =
            serde_json::from_str(&json).expect("Failed to deserialize DriveInfo");
        assert_eq!(drive, deserialized);
    }

    #[test]
    fn test_media_item_serialization() {
        let item = MediaItem {
            id: 101,
            file_hash: "photo_12345_67890".to_string(),
            original_relative_path: "raw/holiday.jpg".to_string(),
            current_relative_path: "albums/Holiday_2026/holiday.jpg".to_string(),
            file_size: 4_512_800,
            mime_type: "image/jpeg".to_string(),
            duration_seconds: None,
            metadata_json: Some(r#"{"camera":"Sony A7IV"}"#.to_string()),
            album_id: Some(12),
            created_at: "2026-10-31 03:00:00".to_string(),
        };

        let json = serde_json::to_string(&item).expect("Failed to serialize MediaItem");
        let deserialized: MediaItem =
            serde_json::from_str(&json).expect("Failed to deserialize MediaItem");
        assert_eq!(item, deserialized);
    }

    #[test]
    fn test_album_serialization() {
        let album = Album {
            id: 12,
            name: "Holiday_2026".to_string(),
            relative_path: "albums/Holiday_2026".to_string(),
            description: Some("Summer vacation photos".to_string()),
            media_count: 85,
            created_at: "2026-10-31 03:00:00".to_string(),
            preview_item: None,
        };

        let json = serde_json::to_string(&album).expect("Failed to serialize Album");
        let deserialized: Album = serde_json::from_str(&json).expect("Failed to deserialize Album");
        assert_eq!(album, deserialized);
    }

    #[test]
    fn test_scan_progress_defaults() {
        let progress = ScanProgress::default();
        assert!(!progress.is_scanning);
        assert_eq!(progress.scanned_count, 0);
        assert_eq!(progress.found_count, 0);
    }
}
