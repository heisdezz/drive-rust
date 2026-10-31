use std::fs;
use std::path::{Path, PathBuf};
use crate::db::{LibraryDatabase, NewMediaItem};
use crate::scanner::walker::DiscoveredMedia;

/// Result of relocating a single media item.
#[derive(Debug, Clone)]
pub struct RelocationOutcome {
    pub media_id: i64,
    pub original_path: String,
    pub relocated_path: String,
    pub album_id: i64,
}

/// Generates a non-colliding filename if `target_path` already exists on disk.
pub fn resolve_collision(target_dir: &Path, original_filename: &str) -> (PathBuf, String) {
    let mut candidate_name = original_filename.to_string();
    let mut candidate_path = target_dir.join(&candidate_name);

    if !candidate_path.exists() {
        return (candidate_path, candidate_name);
    }

    // Split name and extension
    let (stem, ext) = match original_filename.rfind('.') {
        Some(idx) => (&original_filename[..idx], &original_filename[idx..]),
        None => (original_filename, ""),
    };

    let mut counter = 1;
    while candidate_path.exists() {
        candidate_name = format!("{}_{}{}", stem, counter, ext);
        candidate_path = target_dir.join(&candidate_name);
        counter += 1;
    }

    (candidate_path, candidate_name)
}

/// Relocates a discovered media file into `albums/<album_name>/` and writes to SQLite.
pub fn relocate_media(
    mount_path: &Path,
    db: &LibraryDatabase,
    item: &DiscoveredMedia,
) -> Result<RelocationOutcome, String> {
    let albums_dir = mount_path.join("albums");
    let target_album_dir = albums_dir.join(&item.derived_album_name);

    if let Err(e) = fs::create_dir_all(&target_album_dir) {
        return Err(format!("Failed to create album directory: {}", e));
    }

    let original_filename = item
        .absolute_path
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "media_file".to_string());

    let (target_file_path, final_filename) = resolve_collision(&target_album_dir, &original_filename);

    // If file is not already in the target position, move it
    if item.absolute_path != target_file_path {
        // Fast atomic rename (on the same disk)
        if let Err(rename_err) = fs::rename(&item.absolute_path, &target_file_path) {
            // Fall back to copy + delete if cross-device or permission boundary
            if let Err(copy_err) = fs::copy(&item.absolute_path, &target_file_path) {
                return Err(format!("File move failed: {} (copy error: {})", rename_err, copy_err));
            }
            let _ = fs::remove_file(&item.absolute_path);
        }
    }

    // Lookup or create album in DB
    let albums = db.get_albums().map_err(|e| e.to_string())?;
    let album_id = if let Some(existing) = albums.into_iter().find(|a| a.name == item.derived_album_name) {
        existing.id
    } else {
        let new_alb = db
            .create_album(&item.derived_album_name, &item.derived_album_name, None)
            .map_err(|e| e.to_string())?;
        new_alb.id
    };

    let original_rel = item.relative_path.to_string_lossy().to_string();
    let current_rel = format!("{}/{}", item.derived_album_name, final_filename);
    let file_hash = format!("{}_{}", item.file_size, original_filename);

    let new_media = NewMediaItem {
        file_hash: &file_hash,
        original_relative_path: &original_rel,
        current_relative_path: &current_rel,
        file_size: item.file_size as i64,
        mime_type: item.mime_type,
        duration_seconds: None,
        metadata_json: None,
        album_id: Some(album_id),
    };

    let media_id = db.insert_media(&new_media).map_err(|e| e.to_string())?;

    Ok(RelocationOutcome {
        media_id,
        original_path: original_rel,
        relocated_path: current_rel,
        album_id,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_collision() {
        let temp_dir = std::env::temp_dir().join(format!("test_collision_{}", std::process::id()));
        let _ = fs::create_dir_all(&temp_dir);

        // First file doesn't exist
        let (p1, n1) = resolve_collision(&temp_dir, "photo.jpg");
        assert_eq!(n1, "photo.jpg");
        fs::write(&p1, b"first").unwrap();

        // Second file collisions with photo.jpg -> photo_1.jpg
        let (p2, n2) = resolve_collision(&temp_dir, "photo.jpg");
        assert_eq!(n2, "photo_1.jpg");
        fs::write(&p2, b"second").unwrap();

        // Third file collisions with photo.jpg -> photo_2.jpg
        let (_p3, n3) = resolve_collision(&temp_dir, "photo.jpg");
        assert_eq!(n3, "photo_2.jpg");

        let _ = fs::remove_dir_all(&temp_dir);
    }
}
