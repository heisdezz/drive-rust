use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Arc;
use crate::config::AppConfig;
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

    pub fn path(&self) -> &'static str {
        match self {
            AppRoute::Dashboard => "/",
            AppRoute::Discover => "/discover",
            AppRoute::Medias => "/medias",
            AppRoute::Albums => "/albums",
            AppRoute::StateCounter => "/counter",
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
    pub media_origin_route: Option<AppRoute>,

    // Advanced Modals
    pub moving_media_ids: Vec<i64>,
    pub merging_album: Option<Album>,
    pub deleting_album: Option<Album>,
    pub delete_album_permanent: bool,

    // Settings & Configuration State
    pub config: AppConfig,
    pub custom_library_path: Option<PathBuf>,
    pub grid_columns: usize,
    pub show_video_test_modal: bool,
    pub test_video_url: String,
    pub last_audit_report: Option<crate::scanner::audit::LibraryAuditReport>,
}

impl AppState {
    pub fn new() -> Self {
        let config = AppConfig::load();
        let theme_mode = config.general.theme;
        let grid_columns = config.general.grid_columns.clamp(4, 9);
        Self {
            active_route: AppRoute::Dashboard,
            drives: Vec::new(),
            selected_drive: None,
            active_db: None,

            theme_mode,
            theme: AppTheme::from_mode(theme_mode),

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
            media_origin_route: None,

            moving_media_ids: Vec::new(),
            merging_album: None,
            deleting_album: None,
            delete_album_permanent: false,

            config,
            custom_library_path: None,
            grid_columns,
            show_video_test_modal: false,
            test_video_url: "/run/media/destiny/Ventoy/sample.mp4".to_string(),
            last_audit_report: None,
        }
    }

    pub fn set_theme_mode(&mut self, mode: ThemeMode) {
        self.theme_mode = mode;
        self.theme = AppTheme::from_mode(mode);
        self.config.general.theme = mode;
        let _ = self.config.save();
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
        crate::thumbnails::generator::set_playback_active(true);
        self.viewing_media_item = Some(item);
        self.player_prev_id = prev_id;
        self.player_next_id = next_id;
        if self.media_origin_route.is_none() {
            self.media_origin_route = Some(self.active_route);
        }
    }

    pub fn open_player_with_origin(&mut self, item: MediaItem, prev_id: Option<i64>, next_id: Option<i64>, origin: AppRoute) {
        crate::thumbnails::generator::set_playback_active(true);
        self.viewing_media_item = Some(item);
        self.player_prev_id = prev_id;
        self.player_next_id = next_id;
        self.media_origin_route = Some(origin);
    }

    pub fn close_player(&mut self) {
        crate::thumbnails::generator::set_playback_active(false);
        self.viewing_media_item = None;
        self.player_prev_id = None;
        self.player_next_id = None;
        self.media_origin_route = None;
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
                if let Some(first_ext) = drives.iter().find(|d| d.drive_type != DriveType::Internal) {
                    let drive_key = first_ext.mount_path.to_string_lossy().to_string();
                    let cfg = self.config.get_drive_config(&drive_key, &first_ext.id, &first_ext.name);
                    self.custom_library_path = cfg.custom_library_path;
                    self.selected_drive = Some(first_ext.clone());
                    self.reopen_database();
                } else {
                    self.eject_selected_drive();
                }
            }
        } else if self.selected_drive.is_none() {
            // Disabling selecting system drive by default: only auto-select non-internal (external/removable) drives
            if let Some(first_ext) = drives.iter().find(|d| d.drive_type != DriveType::Internal) {
                let drive_key = first_ext.mount_path.to_string_lossy().to_string();
                let cfg = self.config.get_drive_config(&drive_key, &first_ext.id, &first_ext.name);
                self.custom_library_path = cfg.custom_library_path;
                self.selected_drive = Some(first_ext.clone());
                self.reopen_database();
            }
        }

        self.drives = drives;
    }

    pub fn select_drive(&mut self, drive: DriveInfo) {
        if self.selected_drive.as_ref().map(|d| &d.id) != Some(&drive.id) {
            let drive_key = drive.mount_path.to_string_lossy().to_string();
            let cfg = self.config.get_drive_config(&drive_key, &drive.id, &drive.name);
            self.custom_library_path = cfg.custom_library_path;
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
        if let Some(ref drive) = self.selected_drive {
            let drive_key = drive.mount_path.to_string_lossy().to_string();
            let mut cfg = self.config.get_drive_config(&drive_key, &drive.id, &drive.name);
            cfg.custom_library_path = path.clone();
            self.config.set_drive_config(&drive_key, cfg);
            let _ = self.config.save();
        }
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

    /// Returns the resolved absolute PathBuf to the albums directory for the active volume.
    pub fn get_albums_dir(&self) -> PathBuf {
        if let Some(ref drive) = self.selected_drive {
            let drive_key = drive.mount_path.to_string_lossy().to_string();
            let cfg = self.config.get_drive_config(&drive_key, &drive.id, &drive.name);
            let path = PathBuf::from(&cfg.albums_dir);
            if path.is_absolute() {
                path
            } else {
                drive.mount_path.join(path)
            }
        } else if let Some(ref custom) = self.custom_library_path {
            custom.join("albums")
        } else {
            PathBuf::from("albums")
        }
    }

    /// Returns the albums subfolder name or relative path setting for the active volume.
    pub fn get_albums_subfolder_name(&self) -> String {
        if let Some(ref drive) = self.selected_drive {
            let drive_key = drive.mount_path.to_string_lossy().to_string();
            let cfg = self.config.get_drive_config(&drive_key, &drive.id, &drive.name);
            if cfg.albums_dir.trim().is_empty() {
                "albums".to_string()
            } else {
                cfg.albums_dir
            }
        } else {
            "albums".to_string()
        }
    }

    /// Configures and persists the default albums directory for a drive.
    pub fn set_drive_albums_dir(&mut self, drive_key: &str, albums_dir: &str) {
        self.config.set_drive_albums_dir(drive_key, albums_dir);
        let _ = self.config.save();
    }

    /// Sets albums directory for the currently selected drive.
    pub fn set_selected_drive_albums_dir(&mut self, albums_dir: &str) {
        if let Some(ref drive) = self.selected_drive {
            let drive_key = drive.mount_path.to_string_lossy().to_string();
            self.set_drive_albums_dir(&drive_key, albums_dir);
        }
    }

    pub fn set_grid_columns(&mut self, cols: usize) {
        let cols = cols.clamp(4, 9);
        self.grid_columns = cols;
        self.config.general.grid_columns = cols;
        let _ = self.config.save();
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

    pub fn rescan_and_audit(&mut self) -> Option<crate::scanner::audit::LibraryAuditReport> {
        let db = self.active_db.clone()?;
        let mount_path = self.selected_drive.as_ref()?.mount_path.clone();
        let albums_subfolder = self.get_albums_subfolder_name();
        match crate::scanner::audit::audit_and_reconcile_with_albums_dir(&mount_path, &albums_subfolder, &db) {
            Ok(report) => {
                self.last_audit_report = Some(report.clone());
                Some(report)
            }
            Err(err) => {
                eprintln!("Error during library audit: {}", err);
                None
            }
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

#[cfg(test)]
pub mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_theme_mode_cycling() {
        let temp = tempdir().unwrap();
        std::env::set_var("RUST_DRIVE_CONFIG_PATH", temp.path().join("theme_test.json").to_str().unwrap());
        let mut state = AppState::new();
        assert_eq!(state.theme_mode, ThemeMode::ZedDark);

        state.cycle_theme();
        assert_eq!(state.theme_mode, ThemeMode::CatppuccinMocha);

        state.set_theme_mode(ThemeMode::ZedLight);
        assert_eq!(state.theme_mode, ThemeMode::ZedLight);
        std::env::remove_var("RUST_DRIVE_CONFIG_PATH");
    }

    #[test]
    fn test_album_selection_and_modal_state() {
        let mut state = AppState::new();
        assert_eq!(state.selected_album_id, None);
        assert!(!state.show_create_album_modal);

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
        let initial_len = state.user_ignore_patterns.len();

        state.add_ignore_pattern("build_output");
        assert_eq!(state.user_ignore_patterns.len(), initial_len + 1);
        assert!(state.user_ignore_patterns.contains(&"build_output".to_string()));

        // Deduplication check
        state.add_ignore_pattern("build_output");
        assert_eq!(state.user_ignore_patterns.len(), initial_len + 1);

        state.remove_ignore_pattern("build_output");
        assert_eq!(state.user_ignore_patterns.len(), initial_len);
    }

    #[test]
    fn test_player_and_modal_state_management() {
        let mut state = AppState::new();
        assert!(state.viewing_media_item.is_none());
        assert!(state.editing_album.is_none());
        assert!(state.merging_album.is_none());
        assert!(state.deleting_album.is_none());

        let mock_album = Album {
            id: 1,
            name: "Vacation".to_string(),
            relative_path: "albums/Vacation".to_string(),
            description: None,
            media_count: 5,
            created_at: "2026-01-01".to_string(),
            preview_item: None,
        };

        state.open_edit_album_modal(mock_album.clone());
        assert_eq!(state.editing_album.as_ref().map(|a| a.id), Some(1));
        state.close_edit_album_modal();
        assert!(state.editing_album.is_none());

        state.open_merge_modal(mock_album.clone());
        assert_eq!(state.merging_album.as_ref().map(|a| a.id), Some(1));
        state.close_merge_modal();
        assert!(state.merging_album.is_none());

        state.open_delete_modal(mock_album);
        assert_eq!(state.deleting_album.as_ref().map(|a| a.id), Some(1));
        assert!(!state.delete_album_permanent);
        state.set_delete_permanent(true);
        assert!(state.delete_album_permanent);
        state.close_delete_modal();
        assert!(state.deleting_album.is_none());
    }

    #[test]
    fn test_set_drives_disables_selecting_system_by_default() {
        let mut state = AppState::new();

        let sys_drive = DriveInfo {
            id: "sys_1".to_string(),
            name: "System Root".to_string(),
            drive_type: DriveType::Internal,
            mount_path: PathBuf::from("/"),
            size_formatted: "500 GB".to_string(),
            used_percentage: 60,
            is_mounted: true,
        };

        state.set_drives(vec![sys_drive]);
        // System / internal drive should NOT be auto-selected
        assert_eq!(state.selected_drive, None);

        let ext_drive = DriveInfo {
            id: "ext_1".to_string(),
            name: "USB Backup".to_string(),
            drive_type: DriveType::External,
            mount_path: PathBuf::from("/media/backup"),
            size_formatted: "128 GB".to_string(),
            used_percentage: 20,
            is_mounted: true,
        };

        state.set_drives(vec![ext_drive.clone()]);
        // External drive SHOULD be auto-selected
        assert_eq!(state.selected_drive.as_ref().map(|d| &d.id), Some(&ext_drive.id));
    }

    #[test]
    fn test_settings_custom_library_and_grid_density() {
        let temp = tempdir().unwrap();
        std::env::set_var("RUST_DRIVE_CONFIG_PATH", temp.path().join("settings_test.json").to_str().unwrap());
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
        std::env::remove_var("RUST_DRIVE_CONFIG_PATH");
    }

    #[test]
    fn test_per_drive_album_settings_and_config() {
        let temp = tempdir().unwrap();
        let cfg_path = temp.path().join("config_test.json");
        std::env::set_var("RUST_DRIVE_CONFIG_PATH", cfg_path.to_str().unwrap());

        let mut state = AppState::new();
        let drive = DriveInfo {
            id: "drive_abc".to_string(),
            name: "Photos Drive".to_string(),
            drive_type: DriveType::External,
            mount_path: PathBuf::from("/media/photos"),
            size_formatted: "256 GB".to_string(),
            used_percentage: 30,
            is_mounted: true,
        };

        state.select_drive(drive.clone());
        // Default albums dir is "albums"
        assert_eq!(state.get_albums_subfolder_name(), "albums");
        assert_eq!(state.get_albums_dir(), PathBuf::from("/media/photos/albums"));

        // Customize albums location for this drive
        state.set_selected_drive_albums_dir("MyAlbums/Catalog");
        assert_eq!(state.get_albums_subfolder_name(), "MyAlbums/Catalog");
        assert_eq!(state.get_albums_dir(), PathBuf::from("/media/photos/MyAlbums/Catalog"));

        // Reload a new state from the saved config file
        let new_state = AppState::new();
        let loaded_cfg = new_state.config.get_drive_config("/media/photos", "drive_abc", "Photos Drive");
        assert_eq!(loaded_cfg.albums_dir, "MyAlbums/Catalog");

        std::env::remove_var("RUST_DRIVE_CONFIG_PATH");
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
    fn test_media_player_origin_routing() {
        let mut state = AppState::new();
        assert_eq!(state.media_origin_route, None);

        let item = MediaItem {
            id: 99,
            file_hash: "hash".to_string(),
            original_relative_path: "p.jpg".to_string(),
            current_relative_path: "p.jpg".to_string(),
            file_size: 100,
            mime_type: "image/jpeg".to_string(),
            duration_seconds: None,
            metadata_json: None,
            album_id: None,
            created_at: "2026-01-01".to_string(),
        };

        state.open_player_with_origin(item.clone(), None, None, AppRoute::Albums);
        assert_eq!(state.media_origin_route, Some(AppRoute::Albums));
        assert!(state.viewing_media_item.is_some());

        state.close_player();
        assert_eq!(state.media_origin_route, None);
        assert!(state.viewing_media_item.is_none());
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
