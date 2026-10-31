# Implementation Steps: Rust Drive Desktop

> **Porting [[heisdezz/drive](https://github.com/heisdezz/drive)] to Native Rust with GPUI & GPUI-Kit**  
> Complete architecture and context documentation is maintained in Obsidian format under [`context/`](context/00_Index.md).

---

## Core Development Guidelines

1. **Incremental Execution**: Do not one-shot the entire UI or system logic at once. Each step represents a distinct, testable component or module.
2. **Simple Database Logic**: Use direct, explicit `rusqlite` queries with prepared statements and transactions. Avoid over-engineered ORMs.
3. **Component-by-Component UI**: Build and verify individual GPUI / `gpui-kit` components (Sidebar, DriveCard, MediaCard, AlbumCard, Modals) in isolation before composing them into complete views.
4. **Context Maintenance**: Update files in `context/` as each milestone is completed so future agents or developers can seamlessly resume progress.

---

## Phased Implementation Roadmap

### Phase 1: Project Setup & Core Models ✅

- [x] **Step 1.1**: Initialize `Cargo.toml` with `gpui-kit` (0.8.1), `rusqlite`, `sysinfo`, `image`, `tokio`, `serde`, and `chrono`.
- [x] **Step 1.2**: Define domain models in `src/state/models.rs` (`DriveInfo`, `MediaItem`, `Album`, `Tag`, `ScanProgress`). Verify with serialization unit tests.

### Phase 2: Simple SQLite Database Layer (`rusqlite`) ✅

- [x] **Step 2.1**: Implement `src/db/schema.rs` with tables (`media_items`, `albums`, `scan_album_sources`, `tags`, `media_tags`) and triggers maintaining `albums.media_count`. Ensure fallback `unknown` album exists.
- [x] **Step 2.2**: Implement basic queries in `src/db/media.rs` and `src/db/albums.rs` (CRUD, pagination, count maintenance, next/prev seek). Verify with `:memory:` unit tests.

### Phase 3: Removable Drive Detection ✅

- [x] **Step 3.1**: Implement `src/drives/detector.rs` using `sysinfo::Disks`. Filter out system partitions (`/sys`, `/proc`, `/boot`, etc.) and classify drives (`Internal`, `External`, `Network`).
- [x] **Step 3.2**: Add background drive polling loop (`DriveMonitor`) in `src/drives/mod.rs` updating listeners when USB drives are connected or removed.

### Phase 4: GPUI Frame & Component 1: `Sidebar` & `DriveCard` ✅

- [x] **Step 4.1**: Set up GPUI window bootstrap with dark theme and root layout frame in `src/main.rs` and `src/app.rs`.
- [x] **Step 4.2**: Implement `src/components/drive_card.rs` using `gpui-kit` (drive icon, name, formatted size, storage usage bar, mount badge).
- [x] **Step 4.3**: Implement `src/components/sidebar.rs` (navigation links + connected drive cards list). Clicking drives switches active drive in `AppState`.

### Phase 5: Component 2: `MediaCard` & `MediaGrid` (Catalog View)

- [ ] **Step 5.1**: Implement `src/components/media_card.rs` (image thumbnail preview, rounded corners, selection checkbox, video duration badge).
- [ ] **Step 5.2**: Implement `src/components/filter_bar.rs` (debounced search input, type filter tabs `All`/`Images`/`Videos`, sort selector).
- [ ] **Step 5.3**: Implement `src/components/media_grid.rs` with responsive columns and pagination controls. Connect to SQLite database.

### Phase 6: Component 3: `AlbumCard` & `AlbumGrid` (Gallery View)

- [ ] **Step 6.1**: Implement `src/components/album_card.rs` (cover preview, album name, media count badge, context menu trigger).
- [ ] **Step 6.2**: Implement `src/components/views/albums.rs` gallery and `src/components/views/album_detail.rs`.
- [ ] **Step 6.3**: Implement `CreateAlbumModal` and `EditAlbumModal` using `gpui-kit` dialog primitives.

### Phase 7: Component 4: Media Scanner & Relocator

- [ ] **Step 7.1**: Implement `src/scanner/walker.rs` with folder stack traversal and ignore list pattern matching.
- [ ] **Step 7.2**: Implement `src/scanner/relocator.rs` (folder-to-album mapping, atomic move, collision suffixing `photo_1.jpg`, SQLite batch insert).
- [ ] **Step 7.3**: Implement `DiscoverView` scanner UI (folder picker, preset ignore buttons `+ temp`, `+ raw`, live counters, start/pause/stop).

### Phase 8: Component 5: Media Player & Advanced Dialogs

- [ ] **Step 8.1**: Implement `MediaPlayerModal` with high-res photo/video display, arrow key seek navigation, and EXIF metadata panel.
- [ ] **Step 8.2**: Implement `MoveToAlbumModal` and `MergeAlbumModal` with searchable destination picker.
- [ ] **Step 8.3**: Implement `DeleteAlbumModal` (Keep files in Unsorted vs Delete permanently).

### Phase 9: End-to-End Polish & Verification

- [ ] **Step 9.1**: Background thumbnail generator (resizing with `image` crate, video frames via `ffmpeg` CLI) with playback pause gate.
- [ ] **Step 9.2**: Implement `album-rescan` library audit (reconciliation with disk, snapshot recovery from `.backups/`).
- [ ] **Step 9.3**: Full end-to-end testing with removable drive volumes. Verify `cargo test` and `cargo build --release`.
