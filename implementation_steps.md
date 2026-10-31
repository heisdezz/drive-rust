# Implementation Steps: Rust Drive Native Port

## Overview & Methodology

This tracker follows the strict dependency order for the complete native GPUI port of `heisdezz/drive`. Each phase and sub-step must be verified via `cargo check` and unit tests before committing.

---

### Progress Summary

- [x] **Phase 1: Project Setup & GPUI Foundation ✅** (`Cargo.toml`, basic GPUI window, theme tokens)
- [x] **Phase 2: Removable Drive Detection Module ✅** (`sysinfo`-based drive scanner, mock fallback)
- [x] **Phase 3: SQLite Database Schema & Rusqlite Driver ✅** (Schema, queries, migrations, indexes)
- [x] **Phase 4: Component 1 (`Sidebar` & `DriveCard`) ✅** (Responsive sidebar, drive status, navigation)
- [x] **Phase 5: Component 2 (`MediaCard` & `MediaGrid` Catalog View) ✅** (Card layout, filter bar, pagination)
- [x] **Phase 6: Dynamic Theming Engine & Theme Switcher ✅** (5 themes, semantic tokens, quick toggle)
- [x] **Phase 7: Component 3: `AlbumCard` & `AlbumGrid` (Gallery View) ✅** (Album grid, create/edit modals)
- [x] **Phase 8: Component 4: Media Scanner & Relocator ✅** (Iterative crawler, relocator, `DiscoverView`)
- [x] **Phase 9: Component 5: Media Player & Advanced Dialogs ✅** (`MediaPlayerModal`, `MoveToAlbumModal`, `MergeAlbumModal`, `DeleteAlbumModal`)

---

### UI Page-by-Page Polish Roadmap (Current Focus)

> **Design Constraint: Strict Monochrome Iconography**
>
> - **Zero colored emojis** across all pages and modals.
> - Use minimalist, monochrome symbols or Lucide-style line glyphs (`⌕` search, `⊞` folder, `▣` media, `▷` play, `⚙` settings, `×` close, `✓` check, `‹` / `›` chevrons, `↑` / `↓` arrows).
> - All icons inherit theme text tokens (`theme.text_secondary`, `theme.text_muted`, `theme.accent`) without OS-native emoji color overrides.

#### Stage 1: Dashboard View (`dash`) ✅ (Completed)

- [x] **Step UI-1.1**: Build dedicated `src/components/dashboard_view.rs` with clean monochrome iconography.
- [x] **Step UI-1.2**: Storage overview hero card based on reference design:
  - Mounted volume name, total storage, mount path, album folder path.
  - Used percentage progress bar with capacity tracking.
  - Monochrome volume status indicator (`● Mounted`).
  - Action buttons (`Scan Volume`, `Eject`).
- [x] **Step UI-1.3**: Middle row 3 metric cards (`TOTAL MEDIA ITEMS`, `TOTAL ALBUMS`, `STORAGE USED`).
- [x] **Step UI-1.4**: "Recently Added Media" preview strip with direct player launch on click and `View All Media →` navigation.
- [x] **Step UI-1.5**: "Recent Albums" preview card strip with direct album selection on click and `View All Albums →` navigation.
- [x] **Step UI-1.6**: Compact unmounted state with significantly smaller welcome banner and connected drive selector card.

#### Stage 2: Discover & Scanner View (`discover`) ✅ (Completed)

- [x] **Step UI-2.1**: Target drive summary banner with quick scan scope indicators, root path, and destination folder.
- [x] **Step UI-2.2**: Ignore pattern management interface:
  - Monochrome preset pills (`+ temp`, `+ cache`, `+ raw_dumps`, `+ backups`, `+ downloads`, `+ exports`, `+ thumbnails`, `+ node_modules`).
  - Active ignore rules tags with `×` remove icon and hover state.
- [x] **Step UI-2.3**: Live scan monitoring panel:
  - Active scanning state pill (`● Scanning...` / `✓ Scan Complete` / `○ Ready`).
  - Live metric counters: `FILES CRAWLED`, `MEDIA CATALOGED`, `ACTIVE IGNORES`, `STATUS`.
  - Current directory processing label.
- [x] **Step UI-2.4**: Scanner control buttons (`▷ Start Discovery Scan`, `Reset Metrics`).
- [ ] **Step UI-2.5**: Source folder picker: Allow scanning a targeted subfolder (e.g. `DCIM`) instead of only the drive root.
- [ ] **Step UI-2.6**: Scanner lifecycle controls: Pause, Resume, and Cancel/Abort crawl operations.

#### Stage 3: Albums Gallery View (`album`) 🎯 (Current Stage)

- [x] **Step UI-3.0**: Uniform card dimensions & text wrapping:
  - Standardize `AlbumCard` and `MediaCard` to identical height (`180px`) and cover height (`108px`).
  - Multi-line text wrapping for album title, folder relative path, and media filenames with line clamping.
- [ ] **Step UI-3.1**: Gallery toolbar with album search input, total albums count badge, and `+ New Album` button.
- [ ] **Step UI-3.2**: Refined `AlbumCard` with latest media item cover image, item count badge, folder path, and context menu (`Edit`).
- [ ] **Step UI-3.3**: `CreateAlbumModal` integration with preset names and physical disk directory creation.
- [ ] **Step UI-3.4**: Clean empty state when no albums exist.

#### Stage 4: Album Inner / Detail View (`album-inner`)

- [ ] **Step UI-4.1**: Compact breadcrumb bar (`← Albums / <Album Name>`) with item count and storage size.
- [ ] **Step UI-4.2**: Album management trigger (`Edit`) opening edit/merge/delete modals.
- [ ] **Step UI-4.3**: FilterBar scoped to the album (`All`, `Images`, `Videos`, sort criteria, sort direction).
- [ ] **Step UI-4.4**: Album-scoped media grid with card click launching `MediaPlayerModal` with album neighbor navigation.
- [ ] **Step UI-4.5**: Bulk selection toolbar (`Move to Album...`, `Delete Selected`, `Deselect all`).

#### Stage 5: Media Player & Video Test Canvas (`media-player`)

- [ ] **Step UI-5.1**: 38px compact Zed topbar with filename, monochrome type badge, action buttons (`Move`, `Delete`, `Info`, `Close`).
- [ ] **Step UI-5.2**: Sequential neighbor navigation (`‹ Prev` / `› Next`) with keyboard arrow bindings (`Left` / `Right`).
- [ ] **Step UI-5.3**: Image viewer canvas with zoom/fit toggles.
- [ ] **Step UI-5.4**: Video player canvas with monochrome playback controls (play/pause, duration timeline, seeker bar, audio, fullscreen).
- [ ] **Step UI-5.5**: Collapsible EXIF metadata side-drawer (`Info`) with complete camera, lens, exposure, and file hash properties.
- [ ] **Step UI-5.6**: **Video URL Test Page / Modal**: Dedicated test interface with text input to enter external or local `videoUrl` directly, inspect stream codec/container info, and test playback rendering.
- [ ] **Step UI-5.7**: **Local HTTP Media Streaming Server**: Background `127.0.0.1` streaming server supporting HTTP Range requests (`206 Partial Content`) for instant video scrubbing.

#### Stage 6: Media Library Catalog (`medias`)

- [ ] **Step UI-6.1**: Global flat catalog with responsive multi-column card layout.
- [ ] **Step UI-6.2**: Debounced text search filter, media type segmented tabs, and sort criteria.
- [ ] **Step UI-6.3**: Zed pagination bar with total count and page jumping.
- [ ] **Step UI-6.4**: Floating bulk action toolbar (`Move to Album...`, `Delete Selected`, `Tag Selected`, `Export`, `Deselect All`).
- [ ] **Step UI-6.5**: Tags management: Tag assignment modal and filter by tag pills.

#### Stage 7: Settings, Library Location & Drive Management (`settings`)

- [ ] **Step UI-7.1**: Visual theme picker cards with monochrome preview tokens for all 5 themes.
- [ ] **Step UI-7.2**: **Selecting Library Location**: Interface to select, change, or browse a custom library folder location / root mount path instead of only relying on default drive mount points.
- [ ] **Step UI-7.3**: Storage settings, thumbnail cache options (size display & clear cache), and system diagnostic info.
- [x] **Step UI-7.4**: **Sidebar Drive Selection Polish**: Disable selecting System/Internal drives by default on startup (auto-selecting removable/external storage only, or remaining unmounted).
- [ ] **Step UI-7.5**: **Drive Mount & Custom Resource Button**: Sidebar/Settings action button to mount unmounted block devices or attach an embedded/custom filesystem resource.
- [ ] **Step UI-7.6**: **Functional Drive Ejection**: Hook dashboard `Eject` button to safe filesystem unmount routines.
- [ ] **Step UI-7.7**: **Grid Density Slider**: Configurable column grid layout (4 to 9 columns) in Settings.

---

### Phase 10: End-to-End Polish & Verification

- [ ] **Step 10.1**: Background thumbnail generator (resizing with `image` crate, video frames via `ffmpeg` CLI) with playback pause gate.
- [ ] **Step 10.2**: Implement `album-rescan` library audit (reconciliation with disk, orphan cleanup, untracked file discovery).
- [ ] **Step 10.3**: Full end-to-end testing with removable drive volumes. Verify `cargo test` and `cargo build --release`.

---

### Phase 11: Extended Backend Services & Sync

- [ ] **Step 11.1**: **EXIF Metadata Extraction**: Ingest camera, exposure, and GPS data into `metadata_json` using pure Rust EXIF parser.
- [ ] **Step 11.2**: **Database Disaster Recovery (`.backups/`)**: Automatic pre-modification snapshots and restore selector in Settings.
- [ ] **Step 11.3**: **Mobile Sync LAN Server**: Wi-Fi HTTP upload daemon with pairing QR code display in Settings.
