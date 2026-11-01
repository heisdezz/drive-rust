use std::fs;
use std::path::{Path, PathBuf};
use rusqlite::Result;
use crate::db::LibraryDatabase;
use crate::db::NewMediaItem;
use crate::scanner::walker::{is_media_file, get_mime_type};

/// Report generated after library audit and reconciliation.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct LibraryAuditReport {
    pub total_checked: usize,
    pub verified_on_disk: usize,
    pub orphans_cleaned: usize,
    pub untracked_added: usize,
}

/// Audits the SQLite database against the physical storage volume using default "albums" directory.
pub fn audit_and_reconcile(
    mount_path: &Path,
    db: &LibraryDatabase,
) -> Result<LibraryAuditReport> {
    audit_and_reconcile_with_albums_dir(mount_path, "albums", db)
}

/// Audits the SQLite database against the physical storage volume using a configurable albums directory.
/// 1. Verifies that all registered media items actually exist on disk, removing dead/orphan DB records.
/// 2. Scans the configured albums directory for any untracked media files added directly outside the app and catalogs them.
/// 3. Reconciles album media counters to match current items.
pub fn audit_and_reconcile_with_albums_dir(
    mount_path: &Path,
    albums_subfolder: &str,
    db: &LibraryDatabase,
) -> Result<LibraryAuditReport> {
    let mut report = LibraryAuditReport::default();

    // 1. Audit registered media items
    let all_items = db.get_all_media_for_audit()?;
    report.total_checked = all_items.len();

    let mut orphan_ids = Vec::new();
    for (id, rel_path) in all_items {
        let physical = mount_path.join(&rel_path);
        if physical.is_file() {
            report.verified_on_disk += 1;
        } else {
            orphan_ids.push(id);
        }
    }

    if !orphan_ids.is_empty() {
        report.orphans_cleaned = db.delete_media_items(&orphan_ids)?;
    }

    // 2. Discover untracked media directly in the albums directory
    let subfolder = if albums_subfolder.trim().is_empty() {
        "albums"
    } else {
        albums_subfolder.trim()
    };
    let albums_dir = if Path::new(subfolder).is_absolute() {
        PathBuf::from(subfolder)
    } else {
        mount_path.join(subfolder)
    };

    if albums_dir.is_dir() {
        if let Ok(entries) = fs::read_dir(&albums_dir) {
            for entry in entries.flatten() {
                let album_folder = entry.path();
                if album_folder.is_dir() {
                    let album_name = entry.file_name().to_string_lossy().to_string();
                    let rel_album_path = format!("{}/{}", subfolder, album_name);
                    let album = db.get_or_create_album(&album_name, &rel_album_path)?;

                    if let Ok(files) = fs::read_dir(&album_folder) {
                        for file_entry in files.flatten() {
                            let file_path = file_entry.path();
                            if file_path.is_file() && is_media_file(&file_path) {
                                let filename = file_entry.file_name().to_string_lossy().to_string();
                                let rel_path = format!("{}/{}/{}", subfolder, album_name, filename);

                                if !db.media_exists_by_path(&rel_path)? {
                                    let metadata = fs::metadata(&file_path);
                                    let file_size = metadata.map(|m| m.len() as i64).unwrap_or(0);
                                    let mime = get_mime_type(&file_path);
                                    let hash = format!("{:x}_{}", file_size, rel_path);

                                    let new_item = NewMediaItem {
                                        file_hash: &hash,
                                        original_relative_path: &rel_path,
                                        current_relative_path: &rel_path,
                                        file_size,
                                        mime_type: mime,
                                        duration_seconds: None,
                                        metadata_json: None,
                                        album_id: Some(album.id),
                                    };
                                    let _ = db.insert_media(&new_item);
                                    report.untracked_added += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // 3. Reconcile album media counters
    db.recalculate_all_album_counts()?;

    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_audit_cleans_orphans_and_discovers_untracked() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join(".test.db");
        let db = LibraryDatabase::open(&db_path).unwrap();

        // Create an album
        let album = db.get_or_create_album("vacation", "albums/vacation").unwrap();

        // 1. Insert an orphan media item (not existing physically on disk)
        let orphan_item = NewMediaItem {
            file_hash: "orphan1",
            original_relative_path: "albums/vacation/ghost.jpg",
            current_relative_path: "albums/vacation/ghost.jpg",
            file_size: 1024,
            mime_type: "image/jpeg",
            duration_seconds: None,
            metadata_json: None,
            album_id: Some(album.id),
        };
        db.insert_media(&orphan_item).unwrap();

        // 2. Create physical real file directly on disk inside albums/vacation/real.jpg
        let album_dir = dir.path().join("albums").join("vacation");
        fs::create_dir_all(&album_dir).unwrap();
        let real_file = album_dir.join("real.jpg");
        fs::write(&real_file, b"test-jpeg-content").unwrap();

        // Run audit & reconcile
        let report = audit_and_reconcile(dir.path(), &db).unwrap();

        assert_eq!(report.total_checked, 1);
        assert_eq!(report.orphans_cleaned, 1);
        assert_eq!(report.untracked_added, 1);

        // Verify orphan is gone from DB and untracked file is registered
        assert!(!db.media_exists_by_path("albums/vacation/ghost.jpg").unwrap());
        assert!(db.media_exists_by_path("albums/vacation/real.jpg").unwrap());
    }

    #[test]
    fn test_audit_custom_albums_subfolder() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join(".custom_test.db");
        let db = LibraryDatabase::open(&db_path).unwrap();

        let _album = db.get_or_create_album("trips", "MyPhotos/trips").unwrap();
        let album_dir = dir.path().join("MyPhotos").join("trips");
        fs::create_dir_all(&album_dir).unwrap();
        let real_file = album_dir.join("trip.jpg");
        fs::write(&real_file, b"test-trip-photo").unwrap();

        let report = audit_and_reconcile_with_albums_dir(dir.path(), "MyPhotos", &db).unwrap();
        assert_eq!(report.untracked_added, 1);
        assert!(db.media_exists_by_path("MyPhotos/trips/trip.jpg").unwrap());
    }
}
