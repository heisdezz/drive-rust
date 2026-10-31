# Rust Drive - Implementation Roadmap

## Overview

Porting `heisdezz/drive` from Electrobun (SolidJS + WebKit) to native Rust using GPUI (`gpui-kit` `0.8.1`), `rusqlite`, and a compact Zed-editor aesthetic.

---

### Phase 1: Setup & Domain Models ✅

- [x] **Step 1.1**: Initialize workspace and `Cargo.toml` dependencies (`gpui-kit`, `rusqlite`, `sysinfo`, `image`, `tokio`, `serde`, `chrono`).
- [x] **Step 1.2**: Define domain models in `src/state/models.rs` (`DriveInfo`, `MediaItem`, `Album`, `Tag`, `ScanProgress`, filters, sort enums).
- [x] **Step 1.3**: Configure linker (`clang` + system `lld`) and incremental dev caching.

### Phase 2: SQLite Database Layer ✅

- [x] **Step 2.1**: Schema initialization in `src/db/schema.rs` with automatic count triggers and default `unknown` album.
- [x] **Step 2.2**: Album query operations in `src/db/albums.rs` (create, rename, list, count).
- [x] **Step 2.3**: Media item query operations in `src/db/media.rs` (insert, paginated query with filters, neighbor seeking, move).

### Phase 3: Removable Drive Detection ✅

- [x] **Step 3.1**: System mount filtering and removable volume detector in `src/drives/detector.rs` using `sysinfo::Disks`.
- [x] **Step 3.2**: Background drive monitor loop in `src/drives/mod.rs` with periodic polling on GPUI background executor.

### Phase 4: GPUI Frame & Component 1 (`Sidebar` & `DriveCard`) ✅

- [x] **Step 4.1**: Window setup and compact Zed-style layout frame in `src/main.rs` and `src/app.rs`.
- [x] **Step 4.2**: Implement `src/components/drive_card.rs` with disk/USB indicators, volume label, storage progress bar, and percentage badge.
- [x] **Step 4.3**: Implement `src/components/sidebar.rs`: App header, reactive navigation links, connected drives list with selection highlight.

### Phase 5: Component 2 (`MediaCard` & `MediaGrid` Catalog View) ✅

- [x] **Step 5.1**: Implement `src/components/media_card.rs` (aspect ratio thumbnail preview, selection checkbox, video duration badge `▶ MM:SS`, metadata footer).
- [x] **Step 5.2**: Implement `src/components/filter_bar.rs` (segmented type buttons `All`/`Images`/`Videos`, sort criteria `Date`/`Name`/`Size`, direction toggle, item count).
- [x] **Step 5.3**: Implement `src/components/media_grid.rs` with responsive card layout, Zed pagination bar (`Page X of Y`, `Prev`, `Next`, multi-selection). Wired to `AppRoute::Medias`.

### Phase 6: Dynamic Theming Engine & Theme Switcher ✅

- [x] **Step 6.1**: Implement `src/theme/mod.rs` and `src/theme/palettes.rs` with 5 themes (`ZedDark`, `CatppuccinMocha`, `TokyoNight`, `Nord`, `ZedLight`) and semantic tokens.
- [x] **Step 6.2**: Add `theme_mode` and resolved `AppTheme` to `AppState` with reactive `cycle_theme()` and `set_theme_mode()`.
- [x] **Step 6.3**: Add quick-toggle in header and visual `ThemePicker` in Settings. Refactor all components to consume theme tokens.

### Phase 7: Component 3: `AlbumCard` & `AlbumGrid` (Gallery View) ✅

- [x] **Step 7.1: Build `AlbumCard` Component**
  - Implemented `src/components/album_card.rs` with folder cover preview, item count badge, album relative path, and edit trigger (`✎`).
- [x] **Step 7.2: Build `AlbumGrid` View & Detail View**
  - Implemented `src/components/album_grid.rs`:
    - Full album gallery with dynamic counts from SQLite.
    - Album detail view: breadcrumb back bar (`← Albums`), album header, and media card grid.
    - Wired to `AppRoute::Albums` in `src/app.rs`.
- [x] **Step 7.3: Build Album Modals (`CreateAlbumModal`, `EditAlbumModal`)**
  - Implemented `src/components/album_modal.rs`:
    - `render_create_album_modal`: Quick presets (`Vacation`, `Family`, `Screenshots`, `Wallpapers`, etc.), physical disk directory creation, and SQLite insertion.
    - `render_edit_album_modal`: Album deletion with safe fallback to `unknown` album.

---

### Phase 8: Component 4: Media Scanner & Relocator (Next)

- [ ] **Step 8.1: Directory Crawler & Ignore Engine**
  - Implement `src/scanner/walker.rs` with folder stack traversal and ignore list pattern matching.
- [ ] **Step 8.2: Safe Relocator & Collision Resolver**
  - Implement `src/scanner/relocator.rs` (folder-to-album mapping, atomic move, collision suffixing `photo_1.jpg`, SQLite batch insert).
- [ ] **Step 8.3: Build `DiscoverView` Scanner UI**
  - Folder picker, preset ignore buttons (`+ temp`, `+ raw`, `+ cache`), live progress counters, start/pause/stop controls.

---

### Phase 9: Component 5: Media Player & Advanced Dialogs

- [ ] **Step 9.1: Build `MediaPlayerModal`**
  - Full-screen high-res preview, arrow key seek navigation, zoom/pan, and EXIF metadata panel.
- [ ] **Step 9.2: Build `MoveToAlbumModal` & `MergeAlbumModal`**
  - Searchable destination album picker, atomic file moves, database record updates.
- [ ] **Step 9.3: Build `DeleteAlbumModal`**
  - Confirmation dialog with option to keep files in Unsorted vs permanent delete.

---

### Phase 10: End-to-End Polish & Verification

- [ ] **Step 10.1**: Background thumbnail generator (resizing with `image` crate, video frames via `ffmpeg` CLI) with playback pause gate.
- [ ] **Step 10.2**: Implement `album-rescan` library audit (reconciliation with disk, snapshot recovery from `.backups/`).
- [ ] **Step 10.3**: Full end-to-end testing with removable drive volumes. Verify `cargo test` and `cargo build --release`.
