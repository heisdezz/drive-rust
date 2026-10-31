---
title: Implementation Steps & Progress
tags: [roadmap, implementation, tracking]
created: 2026-10-31
---

## Implementation Steps & Progress

This checklist tracks the step-by-step progress of the port from Electrobun to Rust + GPUI.

---

### Core Architecture Phases (Completed)

- [x] **Phase 1: Setup & Domain Models ✅** (`Cargo.toml`, domain models, linker setup)
- [x] **Phase 2: SQLite Database Layer ✅** (Schema, count triggers, `albums` & `media_items` CRUD)
- [x] **Phase 3: Removable Drive Detection ✅** (`sysinfo::Disks`, virtual fs filter, background monitor)
- [x] **Phase 4: GPUI Frame & Component 1 (`Sidebar` & `DriveCard`) ✅** (Window frame, sidebar, drive selector)
- [x] **Phase 5: Component 2 (`MediaCard` & `MediaGrid` Catalog View) ✅** (Card layout, filter bar, pagination)
- [x] **Phase 6: Dynamic Theming Engine & Theme Switcher ✅** (5 themes, semantic tokens, quick toggle)
- [x] **Phase 7: Component 3: `AlbumCard` & `AlbumGrid` (Gallery View) ✅** (Album grid, create/edit modals)
- [x] **Phase 8: Component 4: Media Scanner & Relocator ✅** (Iterative crawler, relocator, `DiscoverView`)
- [x] **Phase 9: Component 5: Media Player & Advanced Dialogs ✅** (`MediaPlayerModal`, `MoveToAlbumModal`, `MergeAlbumModal`, `DeleteAlbumModal`)

---

### UI Page-by-Page Polish Roadmap (Current Focus)

> [!IMPORTANT]
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
- [x] **Step UI-1.4**: \"Recently Added Media\" preview strip with direct player launch on click and `View All Media →` navigation.
- [x] **Step UI-1.5**: \"Recent Albums\" preview card strip with direct album selection on click and `View All Albums →` navigation.
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

#### Stage 3: Albums Gallery View (`album`) 🎯 (Next)

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

#### Stage 5: Media Player & Viewer (`media-player`)

- [ ] **Step UI-5.1**: 38px compact Zed topbar with filename, monochrome type badge, action buttons (`Move`, `Delete`, `Info`, `Close`).
- [ ] **Step UI-5.2**: Sequential neighbor navigation (`‹ Prev` / `› Next`) with keyboard arrow bindings.
- [ ] **Step UI-5.3**: Image viewer canvas with zoom/fit toggles.
- [ ] **Step UI-5.4**: Video player canvas with monochrome playback controls (play/pause, duration timeline, seeker bar, audio, fullscreen).
- [ ] **Step UI-5.5**: Collapsible EXIF metadata side-drawer (`Info`) with complete file and hash properties.

#### Stage 6: Media Library Catalog (`medias`)

- [ ] **Step UI-6.1**: Global flat catalog with responsive multi-column card layout.
- [ ] **Step UI-6.2**: Debounced text search filter, media type segmented tabs, and sort criteria.
- [ ] **Step UI-6.3**: Zed pagination bar with total count and page jumping.
- [ ] **Step UI-6.4**: Multi-select mode with bulk action triggers.

#### Stage 7: Settings & Theming (`settings`)

- [ ] **Step UI-7.1**: Visual theme picker cards with monochrome preview tokens for all 5 themes.
- [ ] **Step UI-7.2**: Storage settings, thumbnail cache options, and system diagnostic info.

---

### Phase 10: End-to-End Polish & Release Verification

- [ ] **Step 10.1**: Background thumbnail generator (resizing with `image` crate, video frames via `ffmpeg` CLI) with playback pause gate.
- [ ] **Step 10.2**: Implement `album-rescan` library audit (reconciliation with disk, snapshot recovery from `.backups/`).
- [ ] **Step 10.3**: Full end-to-end testing with removable drive volumes. Verify `cargo test` and `cargo build --release`.
