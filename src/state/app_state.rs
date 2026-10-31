use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Arc;
use crate::db::LibraryDatabase;
use crate::state::models::{Album, DriveInfo, DriveType, MediaFilterType, MediaSortBy, MediaItem, ScanProgress, SortOrder};
use crate::theme::{AppTheme, ThemeMode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AppRoute {
    #[default]
    Dashboard,
    Discover,
    Medias,
    Albums,
    StateCounter,
    Settings,
    About,
}

impl AppRoute {
    pub fn title(&self) -> &'static str {
        match self {
            AppRoute::Dashboard => "Dashboard",
            AppRoute::Discover => "Discover",
            AppRoute::Medias => "Media Library",
            AppRoute::Albums => "Albums",
            AppRoute::StateCounter => "State Counter",
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
            AppRoute::StateCounter => "/state-counter",
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
    pub album_search_query: String,
    pub show_create_album_modal: bool,
    pub editing_album: Option<Album>,

    // Scanner & Ignore List State
    pub scan_progress: ScanProgress,
    pub user_ignore_patterns: Vec<String>,
    pub scan_subfolder: Option<String>,

    // Media Catalog View State
    pub media_filter: MediaFilterType,
    pub media_sort_by: MediaSortBy,
    pub media_sort_order: SortOrder,
    pub media_search_query: String,
    pub media_page: usize,
    pub media_page_size: usize,
    pub selected_media_ids: HashSet<i64>,

    // Media Player Viewer State
    pub viewing_media_item: Option<MediaItem>,
    pub player_prev_id: Option<i64>,
    pub player_next_id: Option<i64>,
    pub player_show_metadata: bool,
    pub player_zoom_fit: bool,

    // Advanced Modals
    pub moving_media_ids: Vec<i64>,
    pub merging_album: Option<Album>,
    pub deleting_album: Option<Album>,
    pub delete_album_permanent: bool,

    // Settings & Configuration State
    pub custom_library_path: Option<PathBuf>,
    pub grid_columns: usize,
    pub show_video_test_modal: bool,
    pub test_video_url: String,
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
            album_search_query: String::new(),
            show_create_album_modal: false,
            editing_album: None,

            scan_progress: ScanProgress::default(),
            scan_subfolder: None,
            user_ignore_patterns: vec![
                "temp".to_string(),
                "cache".to_string(),
                "raw_dumps".to_string(),
                "thumbnails".to_string(),
            ],

            media_filter: MediaFilterType::All,
            media_sort_by: MediaSortBy::Date,
            media_sort_order: SortOrder::Desc,
            media_search_query: String::new(),
            media_page: 1,
            media_page_size: 24,
            selected_media_ids: HashSet::new(),

            viewing_media_item: None,
            player_prev_id: None,
            player_next_id: None,
            player_show_metadata: true,
            player_zoom_fit: true,

            moving_media_ids: Vec::new(),
            merging_album: None,
            deleting_album: None,
            delete_album_permanent: false,

            custom_library_path: None,
            grid_columns: 6,
            show_video_test_modal: false,
            test_video_url: "/run/media/destiny/Ventoy/sample.mp4".to_string(),
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

    pub fn set_album_search_query(&mut self, query: String) {
        self.album_search_query = query;
    }

    pub fn clear_album_search_query(&mut self) {
        self.album_search_query.clear();
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

    // Media Player actions
    pub fn open_player(&mut self, item: MediaItem, prev_id: Option<i64>, next_id: Option<i64>) {
        self.viewing_media_item = Some(item);
        self.player_prev_id = prev_id;
        self.player_next_id = next_id;
    }

    pub fn close_player(&mut self) {
        self.viewing_media_item = None;
        self.player_prev_id = None;
        self.player_next_id = None;
    }

    pub fn toggle_player_metadata(&mut self) {
        self.player_show_metadata = !self.player_show_metadata;
    }

    pub fn toggle_player_zoom_fit(&mut self) {
        self.player_zoom_fit = !self.player_zoom_fit;
    }

    // Move modal actions
    pub fn open_move_modal(&mut self, ids: Vec<i64>) {
        self.moving_media_ids = ids;
    }

    pub fn close_move_modal(&mut self) {
        self.moving_media_ids.clear();
    }

    // Merge album modal actions
    pub fn open_merge_modal(&mut self, album: Album) {
        self.merging_album = Some(album);
    }

    pub fn close_merge_modal(&mut self) {
        self.merging_album = None;
    }

    pub fn open_merge_album_modal(&mut self, album: Album) {
        self.open_merge_modal(album);
    }

    pub fn close_merge_album_modal(&mut self) {
        self.close_merge_modal();
    }

    // Delete album modal actions
    pub fn open_delete_modal(&mut self, album: Album) {
        self.deleting_album = Some(album);
        self.delete_album_permanent = false;
    }

    pub fn close_delete_modal(&mut self) {
        self.deleting_album = None;
    }

    pub fn open_delete_album_modal(&mut self, album: Album) {
        self.open_delete_modal(album);
    }

    pub fn close_delete_album_modal(&mut self) {
        self.close_delete_modal();
    }

    pub fn set_delete_permanent(&mut self, permanent: bool) {
        self.delete_album_permanent = permanent;
    }

    pub fn set_delete_album_permanent(&mut self, permanent: bool) {
        self.set_delete_permanent(permanent);
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

    pub fn set_scan_subfolder(&mut self, subfolder: Option<String>) {
        self.scan_subfolder = subfolder.filter(|s| !s.trim().is_empty());
    }

    pub fn pause_scan(&mut self) {
        if self.scan_progress.is_scanning {
            self.scan_progress.is_paused = true;
        }
    }

    pub fn resume_scan(&mut self) {
        if self.scan_progress.is_scanning {
            self.scan_progress.is_paused = false;
        }
    }

    pub fn abort_scan(&mut self) {
        self.scan_progress.is_scanning = false;
        self.scan_progress.is_paused = false;
        self.scan_progress.is_stopped = true;
    }

    pub fn set_drives(&mut self, drives: Vec<DriveInfo>) {
        if let Some(selected) = &self.selected_drive {
            if !drives.iter().any(|d| d.id == selected.id) {
                // Previously selected drive is no longer present; pick another external drive or None
                self.selected_drive = drives.iter().find(|d| d.drive_type != DriveType::Internal).cloned();
                self.reopen_database();
            }
        } else if self.selected_drive.is_none() {
            // Disabling selecting system drive by default: only auto-select non-internal (external/removable) drives
            self.selected_drive = drives.iter().find(|d| d.drive_type != DriveType::Internal).cloned();
            self.reopen_database();
        }

        self.drives = drives;
    }

    pub fn select_drive(&mut self, drive: DriveInfo) {
        if self.selected_drive.as_ref().map(|d| &d.id) != Some(&drive.id) {
            self.selected_drive = Some(drive);
            self.selected_album_id = None;
            self.selected_media_ids.clear();
            self.reopen_database();
        }
    }

    pub fn eject_selected_drive(&mut self) {
        self.selected_drive = None;
        self.active_db = None;
        self.selected_album_id = None;
        self.selected_media_ids.clear();
    }

    pub fn set_custom_library_path(&mut self, path: Option<PathBuf>) {
        self.custom_library_path = path.clone();
        if let Some(ref custom) = path {
            let name = custom
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "Custom Library".to_string());
            let custom_drive = DriveInfo {
                id: format!("custom_{}", custom.display()),
                name,
                drive_type: DriveType::External,
                mount_path: custom.clone(),
                size_formatted: "Custom Directory".to_string(),
                used_percentage: 0,
                is_mounted: true,
            };
            self.selected_drive = Some(custom_drive);
            self.selected_album_id = None;
            self.selected_media_ids.clear();
        }
        self.reopen_database();
    }

    pub fn set_grid_columns(&mut self, cols: usize) {
        self.grid_columns = cols.clamp(4, 9);
    }

    pub fn open_video_test_modal(&mut self) {
        self.show_video_test_modal = true;
    }

    pub fn close_video_test_modal(&mut self) {
        self.show_video_test_modal = false;
    }

    pub fn set_test_video_url(&mut self, url: String) {
        self.test_video_url = url;
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

    pub fn set_media_search_query(&mut self, query: String) {
        self.media_search_query = query;
        self.media_page = 1;
    }

    pub fn clear_media_search_query(&mut self) {
        self.media_search_query.clear();
        self.media_page = 1;
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
        let target_db = if let Some(ref custom) = self.custom_library_path {
            Some(custom.join(".media_library.db"))
        } else if let Some(ref drive) = self.selected_drive {
            Some(drive.mount_path.join(".media_library.db"))
        } else {
            None
        };

        if let Some(db_path) = target_db {
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
    fn test_album_search_query_state() {
        let mut state = AppState::new();
        assert_eq!(state.album_search_query, "");

        state.set_album_search_query("vacation".to_string());
        assert_eq!(state.album_search_query, "vacation");

        state.clear_album_search_query();
        assert_eq!(state.album_search_query, "");
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

    #[test]
    fn test_player_and_modal_state_management() {
        let mut state = AppState::new();
        assert!(state.viewing_media_item.is_none());

        let item = MediaItem {
            id: 10,
            file_hash: "hash123".to_string(),
            original_relative_path: "DCIM/img.jpg".to_string(),
            current_relative_path: "albums/unknown/img.jpg".to_string(),
            file_size: 2048,
            mime_type: "image/jpeg".to_string(),
            duration_seconds: None,
            metadata_json: None,
            album_id: Some(1),
            created_at: "2026-10-31 12:00:00".to_string(),
        };

        state.open_player(item.clone(), Some(9), Some(11));
        assert_eq!(state.viewing_media_item.as_ref().map(|i| i.id), Some(10));
        assert_eq!(state.player_prev_id, Some(9));
        assert_eq!(state.player_next_id, Some(11));
        assert!(state.player_show_metadata);

        state.toggle_player_metadata();
        assert!(!state.player_show_metadata);

        state.close_player();
        assert!(state.viewing_media_item.is_none());

        // Test move modal
        state.open_move_modal(vec![10, 11, 12]);
        assert_eq!(state.moving_media_ids.len(), 3);
        state.close_move_modal();
        assert!(state.moving_media_ids.is_empty());
    }

    #[test]
    fn test_set_drives_disables_selecting_system_by_default() {
        use std::path::PathBuf;
        let mut state = AppState::new();
        let sys_drive = DriveInfo {
            id: "sys:/".to_string(),
            name: "Root".to_string(),
            drive_type: DriveType::Internal,
            mount_path: PathBuf::from("/"),
            size_formatted: "500 GB".to_string(),
            used_percentage: 50,
            is_mounted: true,
        };
        state.set_drives(vec![sys_drive]);
        // System drive should NOT be auto-selected by default
        assert!(state.selected_drive.is_none());

        let ext_drive = DriveInfo {
            id: "usb:/mnt/usb".to_string(),
            name: "Sandisk".to_string(),
            drive_type: DriveType::External,
            mount_path: PathBuf::from("/mnt/usb"),
            size_formatted: "64 GB".to_string(),
            used_percentage: 20,
            is_mounted: true,
        };
        state.set_drives(vec![ext_drive.clone()]);
        // External drive SHOULD be auto-selected
        assert_eq!(state.selected_drive.as_ref().map(|d| &d.id), Some(&ext_drive.id));
    }

    #[test]
    fn test_settings_custom_library_and_grid_density() {
        let mut state = AppState::new();
        assert_eq!(state.custom_library_path, None);
        assert_eq!(state.grid_columns, 6);

        let custom = PathBuf::from("/custom/library/folder");
        state.set_custom_library_path(Some(custom.clone()));
        assert_eq!(state.custom_library_path, Some(custom));

        state.set_grid_columns(8);
        assert_eq!(state.grid_columns, 8);

        // Clamping check
        state.set_grid_columns(2);
        assert_eq!(state.grid_columns, 4);

        state.set_grid_columns(20);
        assert_eq!(state.grid_columns, 9);

        // Eject check
        state.eject_selected_drive();
        assert!(state.selected_drive.is_none());
        assert!(state.active_db.is_none());
    }

    #[test]
    fn test_video_test_modal_and_zoom_state() {
        let mut state = AppState::new();
        assert!(!state.show_video_test_modal);
        assert!(state.player_zoom_fit);

        state.open_video_test_modal();
        assert!(state.show_video_test_modal);

        state.set_test_video_url("https://example.com/test.mp4".to_string());
        assert_eq!(state.test_video_url, "https://example.com/test.mp4");

        state.close_video_test_modal();
        assert!(!state.show_video_test_modal);

        state.toggle_player_zoom_fit();
        assert!(!state.player_zoom_fit);
    }

    #[test]
    fn test_media_search_query_state() {
        let mut state = AppState::new();
        state.set_media_page(3);
        assert_eq!(state.media_search_query, "");
        assert_eq!(state.media_page, 3);

        state.set_media_search_query("family".to_string());
        assert_eq!(state.media_search_query, "family");
        assert_eq!(state.media_page, 1);

        state.set_media_page(5);
        state.clear_media_search_query();
        assert_eq!(state.media_search_query, "");
        assert_eq!(state.media_page, 1);
    }
}
