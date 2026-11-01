use std::fs;
use std::path::{Path, PathBuf};
use crate::db::{LibraryDatabase, NewMediaItem};
use crate::scanner::walker::DiscoveredMedia;

/// Outcome of relocating a media file into an album.
#[derive(Debug, Clone)]
pub struct RelocationOutcome {
    pub media_id: i64,
    pub original_path: String,
    pub relocated_path: String,
    pub album_id: i64,
}

/// Computes collision-free destination file path.
/// If `file.jpg` already exists, returns `file_1.jpg`, `file_2.jpg`, etc.
pub fn resolve_collision(target_dir: &Path, filename: &str) -> (PathBuf, String) {
    let mut candidate = target_dir.join(filename);
    if !candidate.exists() {
        return (candidate, filename.to_string());
    }

    let p = Path::new(filename);
    let stem = p.file_stem().and_then(|s| s.to_str()).unwrap_or(filename);
    let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("");

    let mut counter = 1;
    loop {
        let new_filename = if ext.is_empty() {
            format!("{}_{}", stem, counter)
        } else {
            format!("{}_{}.{}", stem, counter, ext)
        };

        candidate = target_dir.join(&new_filename);
        if !candidate.exists() {
            return (candidate, new_filename);
        }
        counter += 1;
    }
}

/// Relocates a discovered media file into `albums/<album_name>/` and writes to SQLite.
pub fn relocate_media(
    mount_path: &Path,
    db: &LibraryDatabase,
    item: &DiscoveredMedia,
) -> Result<RelocationOutcome, String> {
    relocate_media_with_albums_dir(mount_path, "albums", db, item)
}

/// Relocates a discovered media file into a configurable albums directory and writes to SQLite.
pub fn relocate_media_with_albums_dir(
    mount_path: &Path,
    albums_subfolder: &str,
    db: &LibraryDatabase,
    item: &DiscoveredMedia,
) -> Result<RelocationOutcome, String> {
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
        let rel_album_path = format!("{}/{}", subfolder, item.derived_album_name);
        let new_alb = db
            .create_album(&item.derived_album_name, &rel_album_path, None)
            .map_err(|e| e.to_string())?;
        new_alb.id
    };

    let original_rel = item.relative_path.to_string_lossy().to_string();
    let current_rel = format!("{}/{}/{}", subfolder, item.derived_album_name, final_filename);
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
    use tempfile::tempdir;

    #[test]
    fn test_resolve_collision() {
        let dir = tempdir().unwrap();
        let f1 = dir.path().join("photo.jpg");
        fs::write(&f1, b"first").unwrap();

        let (path2, name2) = resolve_collision(dir.path(), "photo.jpg");
        assert_eq!(name2, "photo_1.jpg");
        assert_eq!(path2, dir.path().join("photo_1.jpg"));

        fs::write(&path2, b"second").unwrap();
        let (path3, name3) = resolve_collision(dir.path(), "photo.jpg");
        assert_eq!(name3, "photo_2.jpg");
        assert_eq!(path3, dir.path().join("photo_2.jpg"));
    }
}
