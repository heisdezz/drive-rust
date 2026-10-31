### Phase 6: Dynamic Theming Engine & Theme Switcher (NEW) ✅

- [x] **Step 6.1: Theme Tokens & Color Palette Engine**
  - Implemented `src/theme/mod.rs` and `src/theme/palettes.rs`:
    - `ThemeMode` enum: `ZedDark`, `CatppuccinMocha`, `TokyoNight`, `Nord`, `ZedLight`.
    - Semantic tokens: `bg_canvas`, `bg_sidebar`, `bg_card`, `bg_header`, `bg_pill`, `bg_pill_active`, `border_subtle`, `border_card`, `border_active`, `text_primary`, `text_secondary`, `text_muted`, `accent`, `accent_hover`, `accent_success`, `accent_warning`, `accent_danger`.
- [x] **Step 6.2: AppState Integration & Persistence**
  - Stored `theme_mode` and resolved `AppTheme` tokens on `AppState`.
  - Added `set_theme_mode(&mut self, mode: ThemeMode)` and `cycle_theme(&mut self)` with instant reactive re-rendering.
  - _Verification_: Unit tests for theme cycling and palette resolution passed.
- [x] **Step 6.3: Theme Switcher UI & Global Component Adoption**
  - Implemented quick theme cycle button in the top titlebar/header.
  - Implemented visual `ThemePicker` gallery in `AppRoute::Settings` (`src/components/theme_picker.rs`).
  - Refactored `Sidebar`, `DriveCard`, `MediaCard`, `FilterBar`, `MediaGrid`, and `AppView` to dynamically consume `AppState::theme`.
