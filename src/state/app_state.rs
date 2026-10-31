use std::collections::HashSet;
use std::sync::Arc;
use crate::db::LibraryDatabase;
use crate::state::models::{Album, DriveInfo, MediaFilterType, MediaSortBy, ScanProgress, SortOrder};
use crate::theme::{AppTheme, ThemeMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AppRoute {
    #[default]
    Dashboard,
    Discover,
    Medias,
    Albums,
    Settings,
    About,
}

impl AppRoute {
    pub fn title(&self) -> &'static str {
        match self {
            AppRoute::Dashboard => "Dashboard",
            AppRoute::Discover => "Discover & Scanner",
            AppRoute::Medias => "Media Library",
            AppRoute::Albums => "Albums",
            AppRoute::Settings => "Settings",
            AppRoute::About => "System Info",
        }
    }

    pub fn path_str(&self) -> &'static str {
        match self {
            AppRoute::Dashboard => "/",
            AppRoute::Discover => "/discover",
            AppRoute::Medias => "/medias",
            AppRoute::Albums => "/albums",
            AppRoute::Settings => "/settings",
            AppRoute::About => "/about",
        }
    }
}

pub struct AppState {
    pub active_route: AppRoute,
    pub drives: Vec<DriveInfo>,
    pub selected_drive: Option<DriveInfo>,
    pub active_db: Option<Arc<LibraryDatabase>>,

    // Theme Engine State
    pub theme_mode: ThemeMode,
    pub theme: AppTheme,

    // Album View & Modal State
    pub selected_album_id: Option<i64>,
    pub show_create_album_modal: bool,
    pub editing_album: Option<Album>,

    // Scanner & Ignore List State
    pub scan_progress: ScanProgress,
    pub user_ignore_patterns: Vec<String>,

    // Media Catalog View State
    pub media_filter: MediaFilterType,
    pub media_sort_by: MediaSortBy,
    pub media_sort_order: SortOrder,
    pub media_page: usize,
    pub media_page_size: usize,
    pub selected_media_ids: HashSet<i64>,
}

impl AppState {
    pub fn new() -> Self {
        let default_theme_mode = ThemeMode::ZedDark;
        Self {
            active_route: AppRoute::Dashboard,
            drives: Vec::new(),
            selected_drive: None,
            active_db: None,

            theme_mode: default_theme_mode,
            theme: AppTheme::from_mode(default_theme_mode),

            selected_album_id: None,
            show_create_album_modal: false,
            editing_album: None,

            scan_progress: ScanProgress::default(),
            user_ignore_patterns: vec![
                "temp".to_string(),
                "cache".to_string(),
                "raw_dumps".to_string(),
                "thumbnails".to_string(),
            ],

            media_filter: MediaFilterType::All,
            media_sort_by: MediaSortBy::Date,
            media_sort_order: SortOrder::Desc,
            media_page: 1,
            media_page_size: 24,
            selected_media_ids: HashSet::new(),
        }
    }

    pub fn set_theme_mode(&mut self, mode: ThemeMode) {
        self.theme_mode = mode;
        self.theme = AppTheme::from_mode(mode);
    }

    pub fn cycle_theme(&mut self) {
        let all = ThemeMode::all();
        let current_idx = all.iter().position(|m| *m == self.theme_mode).unwrap_or(0);
        let next_idx = (current_idx + 1) % all.len();
        self.set_theme_mode(all[next_idx]);
    }

    pub fn select_album(&mut self, album_id: Option<i64>) {
        self.selected_album_id = album_id;
        self.media_page = 1;
        self.selected_media_ids.clear();
    }

    pub fn open_create_album_modal(&mut self) {
        self.show_create_album_modal = true;
    }

    pub fn close_create_album_modal(&mut self) {
        self.show_create_album_modal = false;
    }

    pub fn open_edit_album_modal(&mut self, album: Album) {
        self.editing_album = Some(album);
    }

    pub fn close_edit_album_modal(&mut self) {
        self.editing_album = None;
    }

    // Scanner actions
    pub fn add_ignore_pattern(&mut self, pattern: &str) {
        let trimmed = pattern.trim().to_lowercase();
        if !trimmed.is_empty() && !self.user_ignore_patterns.iter().any(|p| p.to_lowercase() == trimmed) {
            self.user_ignore_patterns.push(trimmed);
        }
    }

    pub fn remove_ignore_pattern(&mut self, pattern: &str) {
        self.user_ignore_patterns.retain(|p| p.to_lowercase() != pattern.to_lowercase());
    }

    pub fn update_scan_progress(&mut self, progress: ScanProgress) {
        self.scan_progress = progress;
    }

    pub fn reset_scan_progress(&mut self) {
        self.scan_progress = ScanProgress::default();
    }

    pub fn set_drives(&mut self, drives: Vec<DriveInfo>) {
        // If current selected drive was removed or none selected, pick the first drive
        if let Some(selected) = &self.selected_drive {
            if !drives.iter().any(|d| d.id == selected.id) {
                self.selected_drive = drives.first().cloned();
                self.reopen_database();
            }
        } else if self.selected_drive.is_none() {
            self.selected_drive = drives.first().cloned();
            self.reopen_database();
        }

        self.drives = drives;
    }

    pub fn select_drive(&mut self, drive: DriveInfo) {
        if self.selected_drive.as_ref().map(|d| &d.id) != Some(&drive.id) {
            self.selected_drive = Some(drive);
            self.selected_album_id = None;
            self.reopen_database();
        }
    }

    pub fn set_route(&mut self, route: AppRoute) {
        self.active_route = route;
    }

    pub fn set_media_filter(&mut self, filter: MediaFilterType) {
        self.media_filter = filter;
        self.media_page = 1;
    }

    pub fn set_media_sort_by(&mut self, sort: MediaSortBy) {
        self.media_sort_by = sort;
    }

    pub fn toggle_media_sort_order(&mut self) {
        self.media_sort_order = match self.media_sort_order {
            SortOrder::Asc => SortOrder::Desc,
            SortOrder::Desc => SortOrder::Asc,
        };
    }

    pub fn set_media_page(&mut self, page: usize) {
        self.media_page = page.max(1);
    }

    pub fn toggle_media_selection(&mut self, id: i64) {
        if self.selected_media_ids.contains(&id) {
            self.selected_media_ids.remove(&id);
        } else {
            self.selected_media_ids.insert(id);
        }
    }

    pub fn clear_media_selection(&mut self) {
        self.selected_media_ids.clear();
    }

    fn reopen_database(&mut self) {
        if let Some(drive) = &self.selected_drive {
            let db_path = drive.mount_path.join(".media_library.db");
            match LibraryDatabase::open(&db_path) {
                Ok(db) => {
                    self.active_db = Some(Arc::new(db));
                }
                Err(err) => {
                    eprintln!("Warning: could not open database at {}: {}", db_path.display(), err);
                    self.active_db = None;
                }
            }
        } else {
            self.active_db = None;
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_mode_cycling() {
        let mut state = AppState::new();
        assert_eq!(state.theme_mode, ThemeMode::ZedDark);

        state.cycle_theme();
        assert_eq!(state.theme_mode, ThemeMode::CatppuccinMocha);

        state.set_theme_mode(ThemeMode::Nord);
        assert_eq!(state.theme.name(), "Nord Arctic");
    }

    #[test]
    fn test_album_selection_and_modal_state() {
        let mut state = AppState::new();
        assert_eq!(state.selected_album_id, None);

        state.select_album(Some(42));
        assert_eq!(state.selected_album_id, Some(42));

        state.open_create_album_modal();
        assert!(state.show_create_album_modal);

        state.close_create_album_modal();
        assert!(!state.show_create_album_modal);
    }

    #[test]
    fn test_scanner_ignore_patterns() {
        let mut state = AppState::new();
        assert!(state.user_ignore_patterns.contains(&"temp".to_string()));

        state.add_ignore_pattern("build_artifacts");
        assert!(state.user_ignore_patterns.contains(&"build_artifacts".to_string()));

        state.remove_ignore_pattern("temp");
        assert!(!state.user_ignore_patterns.contains(&"temp".to_string()));
    }
}
