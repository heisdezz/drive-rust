use gpui_kit::*;
use crate::state::app_state::{AppRoute, AppState};
use crate::components::sidebar::render_sidebar;

pub struct AppView {
    app_state: Entity<AppState>,
}

impl AppView {
    pub fn new(app_state: Entity<AppState>, cx: &mut Context<Self>) -> Self {
        // Observe AppState so changes in active_route or drives trigger re-renders
        cx.observe(&app_state, |_, _, cx| {
            cx.notify();
        }).detach();

        Self { app_state }
    }
}

impl Render for AppView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state = self.app_state.read(cx);
        let active_route = state.active_route;
        let selected_drive = state.selected_drive.clone();
        let db_connected = state.active_db.is_some();

        div()
            .flex()
            .w_full()
            .h_full()
            .bg(rgb(0x020617)) // Slate-950 main canvas
            .text_color(rgb(0xf8fafc))
            .font_family(".SystemUIFont")
            // Left: Navigation & Drives Sidebar
            .child(render_sidebar(&self.app_state, window, cx))
            // Right: Main App Canvas
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_grow(1.0)
                    .h_full()
                    .overflow_hidden()
                    // Top App Header & Breadcrumbs
                    .child(
                        div()
                            .h(px(64.0))
                            .px(px(24.0))
                            .flex()
                            .items_center()
                            .justify_between()
                            .bg(rgb(0x0b0f19))
                            .border_b_1()
                            .border_color(rgb(0x1e293b))
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(8.0))
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0xf8fafc))
                                            .child(active_route.title()),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(rgb(0x64748b))
                                            .child("/"),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(rgb(0x64748b))
                                            .child(active_route.path_str()),
                                    ),
                            )
                            // Right Header: Database status indicator
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(12.0))
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap(px(6.0))
                                            .px(px(8.0))
                                            .py(px(3.0))
                                            .rounded(px(12.0))
                                            .bg(rgb(0x1e293b))
                                            .child(
                                                div()
                                                    .w(px(6.0))
                                                    .h(px(6.0))
                                                    .rounded(px(3.0))
                                                    .bg(if db_connected {
                                                        rgb(0x10b981) // Green
                                                    } else {
                                                        rgb(0xf59e0b) // Amber
                                                    }),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .text_color(rgb(0x94a3b8))
                                                    .child(if db_connected {
                                                        "SQLite Ready"
                                                    } else {
                                                        "No DB Mounted"
                                                    }),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .px(px(8.0))
                                            .py(px(3.0))
                                            .rounded(px(4.0))
                                            .bg(rgb(0x1e293b))
                                            .text_xs()
                                            .text_color(rgb(0x60a5fa))
                                            .font_weight(FontWeight::BOLD)
                                            .child("GPUI Native"),
                                    ),
                            ),
                    )
                    // View Content Area
                    .child(
                        div()
                            .flex_grow(1.0)
                            .p(px(24.0))
                            .overflow_y_hidden()
                            .child(render_route_content(active_route, selected_drive.as_ref())),
                    ),
            )
    }
}

/// Renders the content panel for the currently selected route.
fn render_route_content(route: AppRoute, selected_drive: Option<&crate::state::models::DriveInfo>) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap(px(16.0))
        .child(
            div()
                .p(px(20.0))
                .rounded(px(12.0))
                .bg(rgb(0x0f172a))
                .border_1()
                .border_color(rgb(0x1e293b))
                .child(
                    div()
                        .text_lg()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xf8fafc))
                        .child(format!("{} View", route.title())),
                )
                .child(
                    div()
                        .mt(px(6.0))
                        .text_xs()
                        .text_color(rgb(0x94a3b8))
                        .child(match route {
                            AppRoute::Dashboard => "Welcome to Rust Drive. Select an external storage drive from the sidebar to inspect or organize its media library.",
                            AppRoute::Discover => "Scan storage drives, configure folder ignore patterns, and auto-catalog media files into albums.",
                            AppRoute::Medias => "Browse, filter, and search media files indexed on the active external drive.",
                            AppRoute::Albums => "Manage physical album directories, view media counts, and merge or edit albums.",
                            AppRoute::Settings => "Configure LAN Mobile Sync, Google Drive backups, and UI grid column preferences.",
                            AppRoute::About => "Native Rust port powered by GPUI and GPUI-Kit with direct SQLite storage.",
                        }),
                )
                .child(
                    if let Some(drive) = selected_drive {
                        div()
                            .mt(px(16.0))
                            .p(px(12.0))
                            .rounded(px(8.0))
                            .bg(rgb(0x1e293b))
                            .flex()
                            .flex_col()
                            .gap(px(4.0))
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0x60a5fa))
                                    .child(format!("Selected Drive: {}", drive.name)),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x94a3b8))
                                    .child(format!("Mount Path: {}", drive.mount_path.display())),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x94a3b8))
                                    .child(format!("Capacity: {} ({}% used)", drive.size_formatted, drive.used_percentage)),
                            )
                    } else {
                        div()
                            .mt(px(16.0))
                            .text_xs()
                            .text_color(rgb(0xf59e0b))
                            .child("⚠️ No external drive selected. Connect a USB drive or select one from the sidebar.")
                    },
                ),
        )
}
