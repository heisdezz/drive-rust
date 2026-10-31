use std::collections::HashSet;
use std::sync::Arc;
use crate::db::LibraryDatabase;
use crate::state::models::{DriveInfo, MediaFilterType, MediaSortBy, SortOrder};

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
        Self {
            active_route: AppRoute::Dashboard,
            drives: Vec::new(),
            selected_drive: None,
            active_db: None,

            media_filter: MediaFilterType::All,
            media_sort_by: MediaSortBy::Date,
            media_sort_order: SortOrder::Desc,
            media_page: 1,
            media_page_size: 24,
            selected_media_ids: HashSet::new(),
        }
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
