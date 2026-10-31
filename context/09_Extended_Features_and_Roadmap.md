---
title: Extended Features & Comprehensive Roadmap
tags: [features, missing-features, roadmap, specifications, media-server, mobile-sync, tags, exif]
created: 2026-10-31
---

# Extended Features & Comprehensive Roadmap

This document catalogs and specifies all features, backend services, and UI capabilities identified from the reference implementation (`heisdezz/drive`), architectural designs, and system requirements that extend beyond the baseline GPUI prototype.

---

## 1. Local Media Streaming Server (`127.0.0.1`)

### 1.1 Overview & Purpose
In the reference architecture, video playback relies on a local HTTP streaming server bound to `127.0.0.1:<random-port>`. This enables:
- Efficient, memory-safe playback of multi-gigabyte 4K/60fps video files.
- HTTP Range Request (`206 Partial Content`) compliance for immediate scrubbing and seeking without buffering whole files.
- Seamless compatibility with native video decoders and web/embedded players.

### 1.2 Specification
- **Engine**: Lightweight internal HTTP server spawned on Tokio or GPUI background executor.
- **Route**: `GET /stream/:media_id`
  - Resolves `media_id` via SQLite `media_items` table.
  - Inspects `Range` header (`bytes=start-end`).
  - Streams chunked byte slice with `Content-Range: bytes <start>-<end>/<total>` and status `206 Partial Content`.
- **Security**: Localhost only (`127.0.0.1`), token-authenticated request headers, read-only filesystem access restricted to the active drive root.

---

## 2. Mobile Sync LAN Server (`mobile-sync`)

### 2.1 Overview & Purpose
Allows smartphones and remote devices on the same local Wi-Fi network to upload photos and videos directly to the desktop storage drive without cloud intermediaries.

### 2.2 Specification
- **Network Interface**: Binds to local IP (e.g. `192.168.1.X:8080`).
- **Pairing & Discovery**:
  - Settings page generates a transient pairing PIN and monochrome QR code containing `http://<LAN_IP>:<PORT>?token=<AUTH_TOKEN>`.
  - Disconnects / expires when user disables Mobile Sync in Settings.
- **Endpoints**:
  - `GET /api/status`: Returns drive capacity and active library status.
  - `POST /api/upload`: Multipart file upload stream directly writing into `albums/mobile_sync/` (or selected album), validating file hashes and inserting into `.media_library.db`.

---

## 3. EXIF & Metadata Extraction Pipeline

### 3.1 Overview & Purpose
Photo and video files contain critical capture metadata that must be indexed and displayed in the media player side drawer.

### 3.2 Extracted Attributes
- **Camera Details**: Make, model, lens model.
- **Exposure Settings**: ISO, aperture (`f/number`), shutter speed (`exposure_time`), focal length.
- **Temporal & Spatial**: Original capture datetime (`DateTimeOriginal`), GPS latitude/longitude/altitude.
- **Dimensions & Format**: Pixel width, pixel height, color space, codec container.

### 3.3 Implementation
- Uses pure Rust EXIF parser (e.g. `kamadak-exif`) during scanner ingestion.
- Stored as compact JSON in `media_items.metadata_json`.
- Displayed in `MediaPlayerModal` collapsible metadata sidebar.

---

## 4. Tags & Media Organization System

### 4.1 Database Layer
Pre-created schema in `.media_library.db`:
```sql
CREATE TABLE IF NOT EXISTS tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL,
    color_hex TEXT NOT NULL DEFAULT '#3B82F6',
    category TEXT DEFAULT 'General'
);

CREATE TABLE IF NOT EXISTS media_tags (
    media_id INTEGER,
    tag_id INTEGER,
    PRIMARY KEY (media_id, tag_id),
    FOREIGN KEY (media_id) REFERENCES media_items(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);
```

### 4.2 UI Integration
- **Tag Management Modal**: Create, edit name, pick color, or delete tags.
- **Filter Bar Integration**: Tag filter pill dropdown in `FilterBar` to filter media by one or more tags.
- **Media Player & Card Badges**: Display assigned tag pills in the metadata drawer and card overlays.
- **Bulk Tagging**: Assign tags to multiple selected items simultaneously.

---

## 5. Database Snapshots & Disaster Recovery (`.backups/`)

### 5.1 Snapshot Lifecycle
- Prior to any batch operation (mass relocation, album merge, album permanent deletion), copy `.media_library.db` to `.backups/backup_<TIMESTAMP>.db`.
- Retain the 5 most recent snapshots, pruning older backups.
- Provide a `Restore Snapshot` dropdown in Settings if database corruption or unintended deletions occur.

---

## 6. Library Audit & Reconciliation (`album-rescan`)

### 6.1 Audit Logic
1. Crawl physical `albums/` directory on the drive.
2. Cross-reference discovered files against SQLite `media_items.current_relative_path`.
3. **Dead Record Cleanup**: If a record exists in SQLite but the file is gone from disk, prune the record and decrement `albums.media_count`.
4. **Untracked File Ingestion**: If a file exists in `albums/<album_name>/` but has no database record, compute its fingerprint and insert it into the database.
5. **Re-calculate Counts**: Re-synchronize `albums.media_count` with disk reality.

---

## 7. Discover View Scanner Controls & Folder Picker

### 7.1 Custom Source Folder Picker
- In `DiscoverView`, allow the user to select an arbitrary subfolder (e.g. `/run/media/destiny/SanDisk/DCIM/100CANON`) instead of scanning the entire drive root.
- Displays target source path clearly in the scan summary banner.

### 7.2 Active Scan Lifecycle Controls
- **Pause / Resume**: Yield scanner background thread or sleep atomic token.
- **Cancel / Abort**: Safely terminate crawl loop, preserving moves completed up to that point.

---

## 8. Media Player & Testing Suite

### 8.1 Video URL Test Page / Modal
- Dedicated debug route or modal with a text input field to enter a local path or remote `videoUrl`.
- Direct test canvas for video rendering, timescale seeking, play/pause controls, and error fallbacks.

### 8.2 Real Image & Video Canvas Rendering
- Replace placeholder glyphs with real pixel rendering:
  - Images: Decoded with `image` crate and rendered onto GPUI image elements.
  - Video: Framerate-accurate frame rendering or integration with video decoder canvas.
- Keyboard Navigation:
  - `ArrowLeft` / `ArrowRight`: Navigate to previous/next media item.
  - `Space`: Toggle play/pause.
  - `Esc`: Close viewer.
  - `f`: Toggle full window mode.

---

## 9. Media Grid & Bulk Action Toolbar

### 9.1 Live Debounced Text Search
- Search bar input in `FilterBar` filtering filenames, current paths, and metadata in real-time.

### 9.2 Floating Bulk Selection Toolbar
- Appears when `selected_media_ids.len() > 0`:
  - `Move to Album...`: Triggers destination album modal for all selected items.
  - `Delete Selected`: Prompts delete confirmation (keep files vs delete permanently).
  - `Tag Selected`: Multi-select tag assignment.
  - `Export / Download`: Copies selected files to a target directory on the host system.
  - `Deselect All`: Clears selection set.

---

## 10. Drive Management & Settings

### 10.1 Drive Eject & Mount Actions
- Functional execution of OS filesystem unmounting (`umount` on Linux/macOS) upon clicking `Eject`.
- Sidebar action button to mount detected unmounted block storage partitions.
- Interface to choose custom media library directory location instead of default root path.

### 10.2 Settings Enhancements
- **Grid Density Slider**: Configurable column grid layout (4 to 9 columns).
- **Thumbnail Cache Management**: Display disk usage of cached thumbnails with `Clear Thumbnail Cache` button.
- **System Diagnostics Panel**: SQLite database size, schema version, and driver statistics.
