use gpui_kit::*;
use crate::state::app_state::{AppRoute, AppState};
use crate::components::sidebar::render_sidebar;
use crate::components::media_grid::render_media_grid;
use crate::components::theme_picker::render_theme_picker;

pub struct AppView {
    app_state: Entity<AppState>,
}

impl AppView {
    pub fn new(app_state: Entity<AppState>, cx: &mut Context<Self>) -> Self {
        // Observe AppState so changes in active_route, theme, or drives trigger re-renders
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
        let theme = state.theme;

        let state_theme = self.app_state.clone();

        div()
            .flex()
            .w_full()
            .h_full()
            .bg(theme.bg_canvas)
            .text_color(theme.text_primary)
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
                    // Top App Header & Breadcrumbs (Zed 38px titlebar height)
                    .child(
                        div()
                            .h(px(38.0))
                            .px(px(14.0))
                            .flex()
                            .items_center()
                            .justify_between()
                            .bg(theme.bg_header)
                            .border_b_1()
                            .border_color(theme.border_subtle)
                            // Left Breadcrumbs
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(6.0))
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(theme.text_primary)
                                            .child(active_route.title()),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.text_muted)
                                            .child("/"),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.text_secondary)
                                            .child(active_route.path_str()),
                                    ),
                            )
                            // Right Header: Quick Theme Switcher + DB status indicator
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap(px(8.0))
                                    // Quick Theme Cycle Button
                                    .child(
                                        div()
                                            .id(SharedString::from("header-theme-switcher"))
                                            .cursor_pointer()
                                            .flex()
                                            .items_center()
                                            .gap(px(5.0))
                                            .px(px(7.0))
                                            .py(px(2.0))
                                            .rounded(px(4.0))
                                            .bg(theme.bg_pill)
                                            .border_1()
                                            .border_color(theme.border_card)
                                            .hover(move |s| s.bg(theme.bg_pill_active).border_color(theme.border_active))
                                            .on_click(move |_, _, cx| {
                                                state_theme.update(cx, |s, cx| {
                                                    s.cycle_theme();
                                                    cx.notify();
                                                });
                                            })
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .child(theme.mode.icon()),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .text_color(theme.text_secondary)
                                                    .child(theme.mode.name()),
                                            ),
                                    )
                                    // DB Status Indicator
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap(px(5.0))
                                            .px(px(6.0))
                                            .py(px(2.0))
                                            .rounded(px(4.0))
                                            .bg(theme.bg_pill)
                                            .border_1()
                                            .border_color(theme.border_card)
                                            .child(
                                                div()
                                                    .w(px(5.0))
                                                    .h(px(5.0))
                                                    .rounded(px(2.5))
                                                    .bg(if db_connected {
                                                        theme.accent_success
                                                    } else {
                                                        theme.accent_warning
                                                    }),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .font_weight(FontWeight::NORMAL)
                                                    .text_color(theme.text_muted)
                                                    .child(if db_connected {
                                                        "SQLite Ready"
                                                    } else {
                                                        "No DB Mounted"
                                                    }),
                                            ),
                                    )
                                    // GPUI Native Badge
                                    .child(
                                        div()
                                            .px(px(6.0))
                                            .py(px(2.0))
                                            .rounded(px(4.0))
                                            .bg(theme.bg_pill_active)
                                            .border_1()
                                            .border_color(theme.border_active)
                                            .text_xs()
                                            .text_color(theme.accent)
                                            .font_weight(FontWeight::MEDIUM)
                                            .child("GPUI Native"),
                                    ),
                            ),
                    )
                    // View Content Area
                    .child(
                        div()
                            .flex_grow(1.0)
                            .p(px(14.0))
                            .overflow_y_hidden()
                            .child(render_route_content(&self.app_state, active_route, selected_drive.as_ref(), window, cx)),
                    ),
            )
    }
}

/// Renders the content panel for the currently selected route.
fn render_route_content(
    app_state: &Entity<AppState>,
    route: AppRoute,
    selected_drive: Option<&crate::state::models::DriveInfo>,
    window: &mut Window,
    cx: &mut App,
) -> impl IntoElement {
    let state = app_state.read(cx);
    let theme = state.theme;

    match route {
        AppRoute::Medias => div().w_full().h_full().child(render_media_grid(app_state, window, cx)),
        AppRoute::Settings => div()
            .flex()
            .flex_col()
            .gap(px(14.0))
            .w_full()
            .child(
                div()
                    .p(px(14.0))
                    .rounded(px(6.0))
                    .bg(theme.bg_card)
                    .border_1()
                    .border_color(theme.border_card)
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.text_primary)
                            .child("Settings & Preferences"),
                    )
                    .child(
                        div()
                            .mt(px(4.0))
                            .text_xs()
                            .text_color(theme.text_muted)
                            .child("Customize themes, layout density, and network sync configurations."),
                    ),
            )
            .child(
                div()
                    .p(px(14.0))
                    .rounded(px(6.0))
                    .bg(theme.bg_card)
                    .border_1()
                    .border_color(theme.border_card)
                    .child(render_theme_picker(app_state, window, cx)),
            ),
        _ => div()
            .flex()
            .flex_col()
            .gap(px(10.0))
            .child(
                div()
                    .p(px(14.0))
                    .rounded(px(6.0))
                    .bg(theme.bg_card)
                    .border_1()
                    .border_color(theme.border_card)
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.text_primary)
                            .child(format!("{} View", route.title())),
                    )
                    .child(
                        div()
                            .mt(px(4.0))
                            .text_xs()
                            .text_color(theme.text_muted)
                            .child(match route {
                                AppRoute::Dashboard => "Welcome to Rust Drive. Select an external storage drive from the sidebar to inspect or organize its media library.",
                                AppRoute::Discover => "Scan storage drives, configure folder ignore patterns, and auto-catalog media files into albums.",
                                AppRoute::Medias => "Browse, filter, and search media files indexed on the active external drive.",
                                AppRoute::Albums => "Manage physical album directories, view media counts, and merge or edit albums.",
                                AppRoute::Settings => "",
                                AppRoute::About => "Native Rust port powered by GPUI and GPUI-Kit with direct SQLite storage.",
                            }),
                    )
                    .child(
                        if let Some(drive) = selected_drive {
                            div()
                                .mt(px(12.0))
                                .p(px(10.0))
                                .rounded(px(5.0))
                                .bg(theme.bg_pill)
                                .border_1()
                                .border_color(theme.border_card)
                                .flex()
                                .flex_col()
                                .gap(px(3.0))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(theme.accent)
                                        .child(format!("Selected Drive: {}", drive.name)),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_secondary)
                                        .child(format!("Mount Path: {}", drive.mount_path.display())),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_secondary)
                                        .child(format!("Capacity: {} ({}% used)", drive.size_formatted, drive.used_percentage)),
                                )
                        } else {
                            div()
                                .mt(px(12.0))
                                .p(px(10.0))
                                .rounded(px(5.0))
                                .bg(theme.bg_pill)
                                .border_1()
                                .border_color(theme.border_card)
                                .text_xs()
                                .text_color(theme.accent_warning)
                                .child("⚠️ No external drive selected. Connect a USB drive or select one from the sidebar.")
                        },
                    ),
            ),
    }
}
