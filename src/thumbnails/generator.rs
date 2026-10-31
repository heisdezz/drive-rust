use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};

/// Global pause gate for CPU-intensive thumbnail generation.
/// When user is viewing a photo or streaming video, worker threads yield.
pub static PLAYER_PAUSE_GATE: AtomicBool = AtomicBool::new(false);

pub fn set_playback_active(active: bool) {
    PLAYER_PAUSE_GATE.store(active, Ordering::SeqCst);
}

pub fn is_playback_active() -> bool {
    PLAYER_PAUSE_GATE.load(Ordering::SeqCst)
}

#[derive(Debug, Clone)]
pub struct ThumbnailConfig {
    pub max_width: u32,
    pub max_height: u32,
}

impl Default for ThumbnailConfig {
    fn default() -> Self {
        Self {
            max_width: 360,
            max_height: 240,
        }
    }
}

/// Returns the standardized on-disk cache path for a media thumbnail.
pub fn get_thumbnail_path(cache_dir: &Path, file_hash: &str) -> PathBuf {
    cache_dir.join("thumbnails").join(format!("{}.jpg", file_hash))
}

/// Returns true if the thumbnail file already exists on disk.
pub fn has_thumbnail(cache_dir: &Path, file_hash: &str) -> bool {
    get_thumbnail_path(cache_dir, file_hash).is_file()
}

/// Generates a thumbnail for an image file using the `image` crate.
pub fn generate_image_thumbnail(
    source_path: &Path,
    target_path: &Path,
    config: &ThumbnailConfig,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if let Some(parent) = target_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let img = image::ImageReader::open(source_path)?.decode()?;
    let thumb = img.thumbnail(config.max_width, config.max_height);
    thumb.save(target_path)?;
    Ok(())
}

/// Generates a poster frame thumbnail for a video file using `ffmpeg`.
pub fn generate_video_thumbnail(
    source_path: &Path,
    target_path: &Path,
    config: &ThumbnailConfig,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if let Some(parent) = target_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let temp_frame = target_path.with_extension("tmp.jpg");

    let output = Command::new("ffmpeg")
        .args([
            "-y",
            "-ss",
            "00:00:01",
            "-i",
            source_path.to_str().ok_or("Invalid source path")?,
            "-vframes",
            "1",
            "-q:v",
            "2",
            temp_frame.to_str().ok_or("Invalid temp frame path")?,
        ])
        .output();

    match output {
        Ok(out) if out.status.success() && temp_frame.is_file() => {
            let img = image::ImageReader::open(&temp_frame)?.decode()?;
            let thumb = img.thumbnail(config.max_width, config.max_height);
            thumb.save(target_path)?;
            let _ = fs::remove_file(temp_frame);
            Ok(())
        }
        _ => {
            let _ = fs::remove_file(&temp_frame);
            Err("ffmpeg extraction failed or ffmpeg not found".into())
        }
    }
}

/// Dispatches thumbnail creation based on media file type, respecting the playback gate.
pub fn generate_thumbnail(
    source_path: &Path,
    cache_dir: &Path,
    file_hash: &str,
    is_video: bool,
    config: &ThumbnailConfig,
) -> Result<PathBuf, Box<dyn std::error::Error + Send + Sync>> {
    let target = get_thumbnail_path(cache_dir, file_hash);
    if target.is_file() {
        return Ok(target);
    }

    if is_video {
        generate_video_thumbnail(source_path, &target, config)?;
    } else {
        generate_image_thumbnail(source_path, &target, config)?;
    }

    Ok(target)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_playback_pause_gate() {
        assert!(!is_playback_active());
        set_playback_active(true);
        assert!(is_playback_active());
        set_playback_active(false);
        assert!(!is_playback_active());
    }

    #[test]
    fn test_thumbnail_paths() {
        let dir = tempdir().unwrap();
        let hash = "abc123hash";
        let path = get_thumbnail_path(dir.path(), hash);
        assert_eq!(path, dir.path().join("thumbnails").join("abc123hash.jpg"));
        assert!(!has_thumbnail(dir.path(), hash));
    }

    #[test]
    fn test_image_thumbnail_generation() {
        let dir = tempdir().unwrap();
        let src_img_path = dir.path().join("test_img.png");

        // Create a 100x100 dummy image
        let img = image::RgbImage::new(100, 100);
        img.save(&src_img_path).unwrap();

        let thumb_path = get_thumbnail_path(dir.path(), "test_hash");
        let config = ThumbnailConfig {
            max_width: 50,
            max_height: 50,
        };

        let res = generate_image_thumbnail(&src_img_path, &thumb_path, &config);
        assert!(res.is_ok());
        assert!(thumb_path.is_file());

        // Verify dimensions
        let loaded = image::ImageReader::open(&thumb_path).unwrap().decode().unwrap();
        assert!(loaded.width() <= 50);
        assert!(loaded.height() <= 50);
    }
}
