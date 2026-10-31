use std::sync::Arc;
use crate::db::LibraryDatabase;
use crate::state::models::DriveInfo;

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
}

impl AppState {
    pub fn new() -> Self {
        Self {
            active_route: AppRoute::Dashboard,
            drives: Vec::new(),
            selected_drive: None,
            active_db: None,
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
