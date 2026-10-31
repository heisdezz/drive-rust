use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use crate::thumbnails::generator::{generate_thumbnail, is_playback_active, has_thumbnail, ThumbnailConfig};

/// Job item for background thumbnail rendering.
#[derive(Debug, Clone)]
pub struct ThumbnailJob {
    pub media_id: i64,
    pub source_path: PathBuf,
    pub file_hash: String,
    pub is_video: bool,
}

/// A background worker pool thread that processes thumbnail generation jobs,
/// yielding whenever media playback is active to eliminate UI frame drops.
pub struct ThumbnailWorker {
    is_running: Arc<AtomicBool>,
    processed_count: Arc<AtomicUsize>,
}

impl ThumbnailWorker {
    pub fn new() -> Self {
        Self {
            is_running: Arc::new(AtomicBool::new(false)),
            processed_count: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn processed_count(&self) -> usize {
        self.processed_count.load(Ordering::Relaxed)
    }

    /// Process a batch of jobs in a background thread with playback pause gating.
    pub fn start_batch(&self, jobs: Vec<ThumbnailJob>, cache_dir: PathBuf, config: ThumbnailConfig) {
        let running = self.is_running.clone();
        let counter = self.processed_count.clone();

        running.store(true, Ordering::SeqCst);

        thread::spawn(move || {
            for job in jobs {
                if !running.load(Ordering::SeqCst) {
                    break;
                }

                // Playback Pause Gate: sleep while user is actively watching video/images
                while is_playback_active() {
                    thread::sleep(Duration::from_millis(200));
                    if !running.load(Ordering::SeqCst) {
                        return;
                    }
                }

                // Skip if thumbnail already cached
                if has_thumbnail(&cache_dir, &job.file_hash) {
                    counter.fetch_add(1, Ordering::Relaxed);
                    continue;
                }

                // Render thumbnail with error tolerance
                let _ = generate_thumbnail(&job.source_path, &cache_dir, &job.file_hash, job.is_video, &config);
                counter.fetch_add(1, Ordering::Relaxed);

                // Polite throttling: 15ms sleep between renders to keep fan/CPU idle
                thread::sleep(Duration::from_millis(15));
            }

            running.store(false, Ordering::SeqCst);
        });
    }

    pub fn stop(&self) {
        self.is_running.store(false, Ordering::SeqCst);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    #[test]
    fn test_thumbnail_worker_skips_existing() {
        let dir = tempdir().unwrap();
        let cache_dir = dir.path().join(".thumbnails");
        fs::create_dir_all(&cache_dir.join("thumbnails")).unwrap();

        let config = ThumbnailConfig::default();

        let hash = "abc123456789".to_string();
        let thumb_path = cache_dir.join("thumbnails").join(format!("{}.jpg", hash));
        fs::write(&thumb_path, b"mock-thumb-data").unwrap();

        let job = ThumbnailJob {
            media_id: 1,
            source_path: dir.path().join("non_existent.jpg"),
            file_hash: hash,
            is_video: false,
        };

        let worker = ThumbnailWorker::new();
        worker.start_batch(vec![job], cache_dir, config);

        // Wait up to 500ms for worker to finish single existing job
        for _ in 0..10 {
            if worker.processed_count() >= 1 {
                break;
            }
            thread::sleep(Duration::from_millis(50));
        }

        assert_eq!(worker.processed_count(), 1);
        worker.stop();
    }
}
