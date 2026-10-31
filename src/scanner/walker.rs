use std::path::{Path, PathBuf};

/// Common supported media extensions.
const IMAGE_EXTENSIONS: &[&str] = &[
    "jpg", "jpeg", "png", "webp", "gif", "bmp", "tiff", "heic", "avif", "raw", "cr2", "nef",
];

const VIDEO_EXTENSIONS: &[&str] = &[
    "mp4", "mkv", "mov", "avi", "webm", "flv", "wmv", "m4v", "3gp",
];

/// Built-in folder patterns that are always skipped.
const SYSTEM_IGNORED_NAMES: &[&str] = &[
    "albums",
    "node_modules",
    "build",
    "dist",
    "target",
    ".git",
    ".trash",
    ".backups",
    ".media_library.db",
    "system volume information",
    "$recycle.bin",
    "lost+found",
];

/// Checks if a file path is a supported image or video media file.
pub fn is_media_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| {
            let lower = ext.to_lowercase();
            IMAGE_EXTENSIONS.contains(&lower.as_str()) || VIDEO_EXTENSIONS.contains(&lower.as_str())
        })
        .unwrap_or(false)
}

/// Returns the detected MIME type based on extension.
pub fn get_mime_type(path: &Path) -> &'static str {
    match path.extension().and_then(|ext| ext.to_str()).map(|e| e.to_lowercase()).as_deref() {
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("png") => "image/png",
        Some("webp") => "image/webp",
        Some("gif") => "image/gif",
        Some("bmp") => "image/bmp",
        Some("tiff") => "image/tiff",
        Some("heic") => "image/heic",
        Some("avif") => "image/avif",
        Some("mp4") | Some("m4v") => "video/mp4",
        Some("mkv") => "video/x-matroska",
        Some("mov") => "video/quicktime",
        Some("webm") => "video/webm",
        Some("avi") => "video/x-msvideo",
        _ => "application/octet-stream",
    }
}

/// Determines whether a path segment or file should be skipped.
pub fn is_path_ignored(relative_path: &Path, user_ignore_patterns: &[String]) -> bool {
    for component in relative_path.components() {
        let comp_str = component.as_os_str().to_string_lossy();
        let comp_lower = comp_str.to_lowercase();

        // Skip hidden files/directories starting with '.'
        if comp_str.starts_with('.') {
            return true;
        }

        // Skip system directories
        if SYSTEM_IGNORED_NAMES.contains(&comp_lower.as_str()) {
            return true;
        }

        // Check user ignore patterns
        for pattern in user_ignore_patterns {
            let pat_lower = pattern.to_lowercase();
            if comp_lower == pat_lower || comp_lower.contains(&pat_lower) {
                return true;
            }
        }
    }

    false
}

/// Information about a discovered media asset before relocation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscoveredMedia {
    pub absolute_path: PathBuf,
    pub relative_path: PathBuf,
    pub file_size: u64,
    pub mime_type: &'static str,
    pub derived_album_name: String,
}

/// Crawls a directory and discovers all valid, non-ignored media files.
pub fn crawl_directory(
    root: &Path,
    user_ignore_patterns: &[String],
) -> std::io::Result<Vec<DiscoveredMedia>> {
    let mut discovered = Vec::new();
    let mut dir_stack = vec![root.to_path_buf()];

    while let Some(current_dir) = dir_stack.pop() {
        let entries = match std::fs::read_dir(&current_dir) {
            Ok(e) => e,
            Err(_) => continue, // Skip unreadable folders
        };

        for entry in entries.flatten() {
            let path = entry.path();
            let relative = path.strip_prefix(root).unwrap_or(&path);

            if is_path_ignored(relative, user_ignore_patterns) {
                continue;
            }

            if path.is_dir() {
                dir_stack.push(path);
            } else if path.is_file() && is_media_file(&path) {
                let metadata = match entry.metadata() {
                    Ok(m) => m,
                    Err(_) => continue,
                };

                let parent_relative = relative.parent().unwrap_or(Path::new(""));
                let derived_album = if parent_relative.as_os_str().is_empty() {
                    "unknown".to_string()
                } else {
                    parent_relative
                        .components()
                        .next()
                        .map(|c| c.as_os_str().to_string_lossy().to_string())
                        .unwrap_or_else(|| "unknown".to_string())
                };

                discovered.push(DiscoveredMedia {
                    absolute_path: path.clone(),
                    relative_path: relative.to_path_buf(),
                    file_size: metadata.len(),
                    mime_type: get_mime_type(&path),
                    derived_album_name: derived_album,
                });
            }
        }
    }

    Ok(discovered)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_media_file() {
        assert!(is_media_file(Path::new("photo.jpg")));
        assert!(is_media_file(Path::new("video.MP4")));
        assert!(is_media_file(Path::new("image.webp")));
        assert!(!is_media_file(Path::new("document.pdf")));
        assert!(!is_media_file(Path::new("code.rs")));
    }

    #[test]
    fn test_is_path_ignored() {
        let user_patterns = vec!["temp".to_string(), "raw_dumps".to_string()];

        assert!(is_path_ignored(Path::new(".git/HEAD"), &user_patterns));
        assert!(is_path_ignored(Path::new("albums/vacation/pic.jpg"), &user_patterns));
        assert!(is_path_ignored(Path::new("node_modules/pkg/logo.png"), &user_patterns));
        assert!(is_path_ignored(Path::new("temp/snapshot.jpg"), &user_patterns));
        assert!(is_path_ignored(Path::new("photos/raw_dumps/photo.raw"), &user_patterns));

        assert!(!is_path_ignored(Path::new("SummerTrip/beach.jpg"), &user_patterns));
        assert!(!is_path_ignored(Path::new("Family/christmas.mp4"), &user_patterns));
    }
}
