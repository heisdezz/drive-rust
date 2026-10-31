---
title: Implementation Steps & Progress
tags: [roadmap, implementation, tracking]
created: 2026-10-31
---

# Implementation Steps & Progress

This checklist tracks the step-by-step progress of the port from Electrobun to Rust + GPUI.

---

### Phase 1: Setup & Domain Models ✅

- [x] **Step 1.1**: Set up workspace, configure `Cargo.toml` with `gpui-kit` (0.8.1), `rusqlite`, `sysinfo`, `image`, `tokio`, etc.
- [x] **Step 1.2**: Define domain models in `src/state/models.rs` (`DriveInfo`, `MediaItem`, `Album`, `Tag`, `ScanProgress`, etc.).
- [x] **Step 1.3**: Configure linker (`clang` + system `lld`) and incremental dev caching.

---

### Phase 2: SQLite Database Layer ✅

- [x] **Step 2.1**: Implement `src/db/schema.rs` with WAL mode, count triggers, and default `unknown` album.
- [x] **Step 2.2**: Implement `src/db/albums.rs` (CRUD, listing, preview query).
- [x] **Step 2.3**: Implement `src/db/media.rs` (batch insertion, paginated querying, neighbor seeking, moving).

---

### Phase 3: Removable Drive Detection ✅

- [x] **Step 3.1**: Implement `src/drives/detector.rs` with mount filtering and removable drive heuristics.
- [x] **Step 3.2**: Implement `src/drives/mod.rs` with background polling monitor loop.

---

### Phase 4: GPUI Frame & Component 1 (`Sidebar` & `DriveCard`) ✅

- [x] **Step 4.1**: Window setup and layout frame in `src/main.rs` and `src/app.rs`.
- [x] **Step 4.2**: Implement `src/components/drive_card.rs` with volume label, disk/USB indicators, storage progress bar, and percentage badge.
- [x] **Step 4.3**: Implement `src/components/sidebar.rs`: App header, reactive navigation links, connected drives list with selection highlight.

---

### Phase 5: Component 2 (`MediaCard` & `MediaGrid` Catalog View) ✅

- [x] **Step 5.1**: Implement `src/components/media_card.rs` with aspect ratio preview, selection checkbox, video duration badge, and metadata footer.
- [x] **Step 5.2**: Implement `src/components/filter_bar.rs` (segmented type tabs, sort criteria, sort order toggle, total count).
- [x] **Step 5.3**: Implement `src/components/media_grid.rs` with responsive card flow layout, Zed pagination bar, empty state, and multi-select actions. Wired to `AppRoute::Medias`.

---

### Phase 6: Dynamic Theming Engine & Theme Switcher ✅

- [x] **Step 6.1**: Implement `src/theme/mod.rs` and `src/theme/palettes.rs` with 5 curated themes and semantic tokens.
- [x] **Step 6.2**: Add `theme_mode` and resolved `AppTheme` to `AppState` with reactive `cycle_theme()` and `set_theme_mode()`.
- [x] **Step 6.3**: Quick-toggle pill in titlebar and visual `ThemePicker` in Settings. Refactor all components to consume theme tokens.

---

### Phase 7: Component 3: `AlbumCard` & `AlbumGrid` (Gallery View) ✅

- [x] **Step 7.1: Build `AlbumCard` Component**
  - Implemented `src/components/album_card.rs` with cover preview, item count badge, album relative path, and edit trigger (`✎`).
- [x] **Step 7.2: Build `AlbumGrid` View & Detail View**
  - Implemented `src/components/album_grid.rs`:
    - Full album gallery with dynamic counts from SQLite.
    - Album detail view: breadcrumb back bar (`← Albums`), album header, and media card grid.
    - Wired to `AppRoute::Albums` in `src/app.rs`.
- [x] **Step 7.3: Build Album Modals (`CreateAlbumModal`, `EditAlbumModal`)**
  - Implemented `src/components/album_modal.rs`:
    - `render_create_album_modal`: Quick presets, physical disk directory creation, and SQLite insertion.
    - `render_edit_album_modal`: Album deletion with safe fallback to `unknown` album.

---

### Phase 8: Component 4: Media Scanner & Relocator ✅

- [x] **Step 8.1: Directory Crawler & Ignore Engine**
  - Implemented `src/scanner/walker.rs`:
    - Iterative folder traversal with depth protection.
    - Supported extension classification (`jpg`, `jpeg`, `png`, `webp`, `gif`, `mp4`, `mkv`, `mov`, `avi`, etc.).
    - Built-in system ignore list (`albums/`, `.*`, `node_modules`, `tmp`, `cache`, etc.) + user custom patterns.
- [x] **Step 8.2: Safe Relocator & Collision Resolver**
  - Implemented `src/scanner/relocator.rs`:
    - Suffix collision resolution (`photo_1.jpg`, `photo_2.jpg`).
    - Atomic file move with cross-filesystem copy-delete fallback.
    - Automatic album derivation from folder hierarchy.
    - Thread-safe batch SQLite insertions.
- [x] **Step 8.3: Build `DiscoverView` Scanner UI**
  - Implemented `src/components/discover_view.rs`:
    - Target Drive & Mount summary card.
    - Interactive Ignore Pattern manager (preset pills `+ temp`, `+ cache`, etc. and removable active chips `✕`).
    - Real-time scanner metrics card (`Files Crawled`, `Media Cataloged`, `Active Ignore Filters`, `Current Folder`).
    - Async background scanner execution (`cx.spawn`) that keeps UI completely responsive.
    - Wired to `AppRoute::Discover` in `src/app.rs`.

---

### Phase 9: Component 5: Media Player & Advanced Dialogs ✅

- [x] **Step 9.1: Build `MediaPlayerModal`**
  - Implemented `src/components/media_player.rs`:
    - Compact Zed 38px titlebar with filename, type badge, and action buttons (`📁 Move`, `🗑 Delete`, `ℹ Info`, `✕ Close`).
    - Left/Right sequential neighbor navigation (`◀ Prev` / `▶ Next`) seeking records via `db.get_media_item_with_neighbors`.
    - Sleek video player canvas with playback controls mockup (Play/Pause, duration timeline, seeker bar, audio/fullscreen toggles).
    - Image viewer canvas with resolution, size, and MIME badges.
    - Collapsible EXIF metadata drawer showing filename, album path, original source, file size, MIME type, catalog date, full disk path, and SHA-256 hash.
- [x] **Step 9.2: Build `MoveToAlbumModal` & `MergeAlbumModal`**
  - Implemented `src/components/move_modal.rs` (`render_move_to_album_modal`):
    - Target album selector with folder icons and media count metrics.
    - Atomic file move across album directories on disk with collision resolution.
    - Synchronized SQLite catalog update via `update_media_album_and_path`.
    - Wired to bulk action toolbar in `MediaGrid` and `MediaPlayerModal`.
  - Implemented `render_merge_album_modal` in `src/components/album_modal.rs`:
    - Merges all physical files from source album directory into target album directory.
    - Calls `db.merge_albums` and updates all album counts.
- [x] **Step 9.3: Build `DeleteAlbumModal`**
  - Implemented `render_delete_album_modal` in `src/components/album_modal.rs`:
    - Interactive radio selection between:
      1. Safe Preservation (moves files to `unknown` album, preserves physical files on disk).
      2. Permanent Deletion (permanently removes directory from disk and records from SQLite).

---

### Phase 10: End-to-End Polish & Verification (Next)

- [ ] **Step 10.1**: Background thumbnail generator (resizing with `image` crate, video frames via `ffmpeg` CLI) with playback pause gate.
- [ ] **Step 10.2**: Implement `album-rescan` library audit (reconciliation with disk, snapshot recovery from `.backups/`).
- [ ] **Step 10.3**: Full end-to-end testing with removable drive volumes. Verify `cargo test` and `cargo build --release`.
