---

### Phase 5: Component 2: `MediaCard` & `MediaGrid` (Catalog View) ✅

- [x] **Step 5.1: Build `MediaCard` Component**
  - Implemented `src/components/media_card.rs`:
    - Image thumbnail preview container with rounded corners and dark Zed aesthetic.
    - Multi-select checkbox badge with active highlight.
    - Video duration badge overlay (`▶ 01:24`).
  - _Verification_: Unit tests for `format_duration` passed.
- [x] **Step 5.2: Build `FilterBar` Component**
  - Implemented `src/components/filter_bar.rs`:
    - Segmented media type buttons (`All`, `Images`, `Videos`).
    - Sort dropdown (`Date`, `Name`, `Size`) and sort direction toggle (`▲ Asc`/`▼ Desc`).
    - Total catalog count badge.
  - _Verification_: Filter toolbar emits state updates to `AppState`.
- [x] **Step 5.3: Build `MediaGrid` & Pagination**
  - Implemented `src/components/media_grid.rs`:
    - Responsive card layout with selection badges.
    - Compact Zed pagination bar (Page X of Y, Previous, Next, Deselect all).
    - Connects directly to `LibraryDatabase::get_media_items`.
  - _Verification_: Compiles cleanly; wired into `AppView` under `AppRoute::Medias`.

---
